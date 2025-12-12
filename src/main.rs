#![allow(static_mut_refs)]
extern crate glfw;

use gl::types::{GLint, GLuint};
use glfw::{
    Action, Context, Key,
    ffi::{glfwDestroyWindow, glfwSetWindowMonitor},
};
use nalgebra_glm as glm;
use std::ffi::CString;

mod camera;
mod mousehandler;
mod shader;
use crate::{camera::Camera3d, mousehandler::MouseHandler, shader::VertexArrayObject};

// =============================================================
// ============= Helper Functions (Uniforms & Shaders) =========
// =============================================================

fn get_uniform_location(program: GLuint, name: &str) -> i32 {
    let c_name = CString::new(name)
        .map_err(|_| format!("Failed to create CString for uniform: {}", name))
        .unwrap();

    unsafe { gl::GetUniformLocation(program, c_name.as_ptr()) }
}

fn set_mat4_uniform(program: GLuint, name: &str, m: &glm::Mat4) -> Result<(), String> {
    let location = get_uniform_location(program, name);

    if location == -1 {
        return Ok(());
    }

    unsafe { gl::UniformMatrix4fv(location, 1, gl::FALSE, m.as_ptr()) };
    Ok(())
}

fn set_vec3_uniform(program: GLuint, name: &str, v: &glm::Vec3) -> Result<(), String> {
    let location = get_uniform_location(program, name);

    if location == -1 {
        return Ok(());
    }

    unsafe {
        gl::Uniform3fv(location, 1, v.as_ptr());
    }
    Ok(())
}

// =============================================================
// ======================= Main Program ========================
// =============================================================

const KEY_COUNT: usize = 1024;
static mut KEY_STATES: [bool; KEY_COUNT] = [false; KEY_COUNT];
static mut FULLSCREEN: bool = false;

static mut POS_WINDOWED: (i32, i32) = (-1, -1);
static mut SIZE_WINDOWED: (i32, i32) = (-1, -1);

fn main() {
    // ------------------------ GLFW Init ------------------------
    glfw::init_hint(glfw::InitHint::Platform(glfw::Platform::Wayland));
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let width = 1080.0f32;
    let height = 720.0f32;

    let (mut window, events) = glfw
        .create_window(
            width as u32,
            height as u32,
            "ZephyrosOpenGLGame",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_all_polling(true);

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
        width / height,
        0.1,
        100.0,
    );

    // ------------------------- Load GL -------------------------
    gl::load_with(|s| {
        window
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

    let mut last_frame: f32 = 0.0;
    let mut delta_time: f32 = 0.0;

    let (width, height) = window.get_size();
    unsafe { gl::Viewport(0, 0, width, height) };
    camera.aspect_ratio = width as f32 / height as f32;
    let mut mousehandler = MouseHandler::new(width as f32, height as f32);

    window.set_cursor_mode(glfw::CursorMode::Disabled);

    while !window.should_close() {
        // ----- DELTA TIME -----
        let current_time = glfw.get_time() as f32;
        delta_time = current_time - last_frame;
        last_frame = current_time;

        glfw.poll_events();

        // ------------------------ Input -------------------------
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(key, _, action, _) => unsafe {
                    if (key as usize) < KEY_COUNT {
                        match action {
                            Action::Press => KEY_STATES[key as usize] = true,
                            Action::Release => KEY_STATES[key as usize] = false,
                            _ => {}
                        }
                    }

                    if key == Key::Escape && action == Action::Press {
                        window.set_should_close(true);
                    }
                    if key == Key::LeftAlt && action == Action::Press {
                        mousehandler.locked = !mousehandler.locked;
                        if mousehandler.locked {
                            window.set_cursor_mode(glfw::CursorMode::Disabled); // Captures and hides cursor
                        } else {
                            window.set_cursor_mode(glfw::CursorMode::Normal); // Captures and hides cursor
                        }
                    }
                    if key == Key::F11 && action == Action::Press {
                        unsafe {
                            FULLSCREEN = !FULLSCREEN;
                            if FULLSCREEN {
                                // Save current windowed position and size
                                POS_WINDOWED = window.get_pos();
                                SIZE_WINDOWED =
                                    (window.get_size().0 as i32, window.get_size().1 as i32);

                                // Get primary monitor
                                glfw.with_primary_monitor(|_, monitor| {
                                    if let Some(monitor) = monitor {
                                        let video_mode = monitor
                                            .get_video_mode()
                                            .expect("Failed to get video mode");
                                        window.set_monitor(
                                            glfw::WindowMode::FullScreen(monitor),
                                            0,
                                            0,
                                            video_mode.width,
                                            video_mode.height,
                                            Some(video_mode.refresh_rate),
                                        );
                                    }
                                });
                            } else {
                                // Restore windowed mode
                                window.set_monitor(
                                    glfw::WindowMode::Windowed,
                                    POS_WINDOWED.0,
                                    POS_WINDOWED.1,
                                    SIZE_WINDOWED.0 as u32,
                                    SIZE_WINDOWED.1 as u32,
                                    None,
                                );
                            }
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
        unsafe {
            let mut direction = glm::vec3(0.0, 0.0, 0.0);

            if KEY_STATES[Key::W as usize] {
                direction += camera.front;
            }
            if KEY_STATES[Key::S as usize] {
                direction -= camera.front;
            }
            if KEY_STATES[Key::A as usize] {
                direction -= camera.right;
            }
            if KEY_STATES[Key::D as usize] {
                direction += camera.right;
            }
            if KEY_STATES[Key::Space as usize] {
                direction += camera.up;
            }
            if KEY_STATES[Key::LeftShift as usize] {
                direction -= camera.up;
            }
            if KEY_STATES[Key::Up as usize] {
                camera.process_mouse(0.0, 1.0 * delta_time * 50.0 / camera.mouse_sensitivity * 2.);
            }
            if KEY_STATES[Key::Down as usize] {
                camera.process_mouse(
                    0.0,
                    -1.0 * delta_time * 50.0 / camera.mouse_sensitivity * 2.,
                );
            }
            if KEY_STATES[Key::Left as usize] {
                camera.process_mouse(
                    -1.0 * delta_time * 50.0 / camera.mouse_sensitivity * 2.,
                    0.0,
                );
            }
            if KEY_STATES[Key::Right as usize] {
                camera.process_mouse(1.0 * delta_time * 50.0 / camera.mouse_sensitivity * 2., 0.0);
            }

            if glm::length(&direction) > 0.0 {
                camera.position += direction * camera.move_speed * delta_time;
            }
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

        window.swap_buffers();
    }
}
