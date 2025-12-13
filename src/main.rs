#![allow(static_mut_refs)]
extern crate glfw;
use glfw::{Action, Context, Key};
use nalgebra_glm as glm;

mod camera;
mod mousehandler;
mod shader;
mod windowing;
mod part;
use crate::{
    camera::Camera3d,
    mousehandler::MouseHandler,
    shader::VertexArrayObject,
    windowing::{GameWindow, GameWindowHints},
};
use log::{debug, error, info, trace, warn};
use log4rs;

// =============================================================
// ======================= Main Program ========================
// =============================================================

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    debug!(
        "{}",
        format!("Launched BUAT v{}", env!("CARGO_PKG_VERSION"))
    );
    let mut gameWindow = GameWindow::new(GameWindowHints {
        gl_context: (3, 3),
        profile: glfw::OpenGlProfileHint::Core,
        title: format!("BUAT {}", env!("CARGO_PKG_VERSION")).as_str(),
        fullscreen: false,
        size: (1080, 720),
    })
    .unwrap();

    // -------------------- Camera Initialization --------------------
    let mut camera = Camera3d::new(
        glm::vec3(0., 0., 3.),
        glm::vec3(0., 1., 0.),
        -0.0,
        0.0,
        0.15,
        1.5,
        45.0,
        true,
        gameWindow.win.get_size().0 as f32 / gameWindow.win.get_size().1 as f32,
        0.1,
        100.0,
    );

    // ------------------------- Load GL -------------------------
    gl::load_with(|s| {
        gameWindow
            .win
            .get_proc_address(s)
            .map(|p| p as *const _)
            .unwrap_or(std::ptr::null())
    });
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    // ------------------------ Shaders --------------------------
    let vertex_shader_src = r#"
        #version 330 core
        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aColor;

        uniform mat4 view;
        uniform mat4 projection;

        out vec3 ourColor;

        void main() {
            gl_Position = projection * view * vec4(aPos, 1.0);
            ourColor = aColor;
        }
    "#;

    let fragment_shader_src = r#"
        #version 330 core
        in vec3 ourColor;
        out vec4 FragColor;

        void main() {
            FragColor = vec4(ourColor, 1.0);
        }
    "#;

    let shader = shader::Shader::new(vertex_shader_src, fragment_shader_src);

    // -------------------------- Geometry --------------------------
    let vertices: [f32; 18] = [
        // positions        // colors
        -0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom-left
        0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom-right
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
    ];

    // The triangle's indices
    let indices: [u32; 3] = [0, 1, 2];

    let (mut vbo, mut ebo) = (0, 0);
    let mut vao = VertexArrayObject::new();

    unsafe {
        gl::GenVertexArrays(1, &mut vao.id);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao.id);

        // -------- VBO --------
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as _,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // -------- EBO --------
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as _,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // -------- Vertex Attributes --------
        let stride = 6 * std::mem::size_of::<f32>() as i32;

        // Position (location = 0)
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // Color (location = 1)
        gl::VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (3 * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);

        gl::BindVertexArray(0);
    }

    vao.bind();
    vao.unbind();

    // =============================================================
    // ========================= Main Loop =========================
    // =============================================================

    let mut mousehandler = MouseHandler::new(0., 0.);

    gameWindow.win.set_cursor_mode(glfw::CursorMode::Disabled);

    debug!("Starting main loop...");
    while !gameWindow.win.should_close() {
        // ----- DELTA TIME -----
        let delta_time = gameWindow.tick();

        gameWindow.glfw.poll_events();

        // ------------------------ Input -------------------------
        for (_, event) in glfw::flush_messages(&gameWindow.ev) {
            match event {
                glfw::WindowEvent::Key(key, _, action, _) => unsafe {
                    if (key as usize) < windowing::KEY_COUNT {
                        match action {
                            Action::Press => gameWindow.key_states[key as usize] = true,
                            Action::Release => gameWindow.key_states[key as usize] = false,
                            _ => {}
                        }
                    }

                    if key == Key::Escape && action == Action::Press {
                        gameWindow.win.set_should_close(true);
                    }
                    if key == Key::LeftAlt && action == Action::Press {
                        mousehandler.locked = !mousehandler.locked;
                        if mousehandler.locked {
                            gameWindow.win.set_cursor_mode(glfw::CursorMode::Disabled); // Captures and hides cursor
                        } else {
                            gameWindow.win.set_cursor_mode(glfw::CursorMode::Normal); // Captures and hides cursor
                        }
                    }
                },
                glfw::WindowEvent::Size(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) };
                    camera.aspect_ratio = width as f32 / height as f32;
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) };
                    camera.aspect_ratio = width as f32 / height as f32;
                }
                glfw::WindowEvent::CursorPos(xpos, ypos) => {
                    let (x, y) = mousehandler.handle_mouse(xpos as f32, ypos as f32);
                    if mousehandler.locked {
                        camera.process_mouse(x, y)
                    };
                }

                _ => {}
            }
        }

        // Poll KEY_STATES for smooth movement (dont put in the thingy above)

        let mut direction = glm::vec3(0.0, 0.0, 0.0);
        let key_states = gameWindow.key_states;

        if key_states[Key::W as usize] {
            direction += camera.front;
        }
        if key_states[Key::S as usize] {
            direction -= camera.front;
        }
        if key_states[Key::A as usize] {
            direction -= camera.right;
        }
        if key_states[Key::D as usize] {
            direction += camera.right;
        }
        if key_states[Key::Space as usize] {
            direction += camera.up;
        }
        if key_states[Key::LeftShift as usize] {
            direction -= camera.up;
        }
        if key_states[Key::Up as usize] {
            camera.process_mouse(0.0, 1.0 * delta_time * 50.0 / camera.mouse_sensitivity * 2.);
        }
        if key_states[Key::Down as usize] {
            camera.process_mouse(
                0.0,
                -1.0 * delta_time * 50.0 / camera.mouse_sensitivity * 2.,
            );
        }
        if key_states[Key::Left as usize] {
            camera.process_mouse(
                -1.0 * delta_time * 50.0 / camera.mouse_sensitivity * 2.,
                0.0,
            );
        }
        if key_states[Key::Right as usize] {
            camera.process_mouse(1.0 * delta_time * 50.0 / camera.mouse_sensitivity * 2., 0.0);
        }

        if glm::length(&direction) > 0.0 {
            camera.position += direction * camera.move_speed * delta_time;
        }

        // ----------------------- Rendering -----------------------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.use_program();

            let view = camera.get_view_matrix();
            let projection = camera.get_projection_matrix();

            shader.set_mat4("view", &view).unwrap();
            shader.set_mat4("projection", &projection).unwrap();

            vao.bind();
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, std::ptr::null());
        }

        gameWindow.win.swap_buffers();
    }
    debug!("Closed")
}
