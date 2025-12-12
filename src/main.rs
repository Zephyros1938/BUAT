#![allow(static_mut_refs)]
extern crate glfw;
use glfw::{Action, Context, Key};
use nalgebra_glm as glm;

mod camera;
mod mousehandler;
mod shader;
mod windowing;
use crate::{
    camera::Camera3d,
    mousehandler::MouseHandler,
    shader::VertexArrayObject,
    windowing::{GameWindow, GameWindowHints},
};

// =============================================================
// ======================= Main Program ========================
// =============================================================

fn main() {
    let mut gameWindow = GameWindow::new(GameWindowHints {
        gl_context: (3, 3),
        profile: glfw::OpenGlProfileHint::Core,
        title: "BUAT",
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

    // -------------------------- Geometry -----------------------
    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // bottom-left
        0.5, -0.5, 0.0, // bottom-right
        0.0, 0.5, 0.0, // top
    ];

    let colors: [f32; 9] = [
        1.0, 0.0, 0.0, // red
        0.0, 1.0, 0.0, // green
        0.0, 0.0, 1.0, // blue
    ];

    let (mut vbo_pos, mut vbo_color) = (0, 0);
    let mut vao = VertexArrayObject::new();

    unsafe {
        gl::GenVertexArrays(1, &mut vao.id);
        gl::GenBuffers(1, &mut vbo_pos);
        gl::GenBuffers(1, &mut vbo_color);

        gl::BindVertexArray(vao.id);

        // Position buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_pos);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as _,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        // Color buffer
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_color);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (colors.len() * std::mem::size_of::<f32>()) as _,
            colors.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 0, std::ptr::null());
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    vao.bind();
    vao.unbind();

    // =============================================================
    // ========================= Main Loop =========================
    // =============================================================

    let mut mousehandler = MouseHandler::new(0., 0.);

    gameWindow.win.set_cursor_mode(glfw::CursorMode::Disabled);

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
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        gameWindow.win.swap_buffers();
    }
}
