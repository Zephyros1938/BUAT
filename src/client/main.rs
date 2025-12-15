use std::env;
use std::process::exit;

use glfw::{Action, Context, Key};
use nalgebra_glm as glm;
use log::{debug, info};
use log4rs;
use mini_redis::client;
use tokio::runtime;
use tokio::time::{Duration, sleep};

mod graphics;
mod object;
mod input;

use {
    graphics::{
        camera::Camera3d,
        shader,
        windowing::{self, GameWindow, GameWindowHints},
    },
    input::mousehandler::MouseHandler,
    object::{base::Render, mesh_loader, part::Part},
};

// =============================================================
// ====================== Server Connect =======================
// =============================================================

async fn preconnect(uri: &String) {
    if let Ok(mut client) = client::connect(uri).await {
        let result = client.get("ping").await;
        println!("Server pinged. Result: {:?}", result);
    } else {
        println!("Can't connect to server!");
        exit(1);
    }
}

// =============================================================
// ======================= Main Program ========================
// =============================================================

#[tokio::main]
async fn main() {
    // ------------------------ Logging -------------------------
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let _x = mesh_loader::load_vertices_from_obj("assets/test.obj").1;
    info!("{:?}", _x);

    debug!(
        "{}",
        format!("Launched BUAT v{}", env!("CARGO_PKG_VERSION"))
    );


    // -------------------- Connect to Server --------------------
    let env_args: Vec<String> = env::args().collect();
    debug!("Client args: {:?}", env_args);

    let uri_default: String = String::from("127.0.0.1:6700");
    let uri: String = env_args
        .get(2)
        .unwrap_or(&uri_default)
        .to_string();

    let sid: u64 = env_args
        .get(1)
        .expect("No ID specified!")
        .parse::<u64>()
        .expect("Invalid ID!");
    
    preconnect(&uri).await;
    
    // ------------------------- Window -------------------------
    let mut game_window = GameWindow::new(GameWindowHints {
        gl_context: (3, 3),
        profile: glfw::OpenGlProfileHint::Core,
        title: format!("BUAT {}", env!("CARGO_PKG_VERSION")).as_str(),
        fullscreen: false,
        size: (1080, 720),
    })
    .unwrap();

    // ------------------------- Load GL -------------------------
    gl::load_with(|s| {
        game_window
            .win
            .get_proc_address(s)
            .map(|p| p as *const _)
            .unwrap_or(std::ptr::null())
    });
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE); // Enable face culling
        gl::CullFace(gl::BACK); // Cull back-facing polygons
        gl::FrontFace(gl::CCW); // Counter-clockwise winding = front face
    }

    // ------------------------ Shaders --------------------------
    let vertex_shader_src = r#"
        #version 330 core

        layout (location = 0) in vec3 aPos;
        layout (location = 1) in vec3 aColor;

        out vec3 ourColor;
        out float fragDistance;

        uniform mat4 model;
        uniform mat4 view;
        uniform mat4 projection;

        void main()
        {
            vec4 worldPos = model * vec4(aPos, 1.0);
            vec4 viewPos  = view * worldPos;

            fragDistance = length(viewPos.xyz);
            ourColor = aColor;

            gl_Position = projection * viewPos;
        }

    "#;

    let fragment_shader_src = r#"
        #version 330 core

        in vec3 ourColor;
        in float fragDistance;

        out vec4 FragColor;

        /* Fog constants */
        const vec3  FOG_COLOR = vec3(0.0, 0.0, 0.0);
        const float FOG_START = 0.0;
        const float FOG_END   = 20.0;

        void main()
        {
            float fogFactor = clamp(
                (FOG_END - fragDistance) / (FOG_END - FOG_START),
                0.0,
                1.0
            );

            vec3 color = mix(FOG_COLOR, ourColor, fogFactor);
            FragColor = vec4(color, 1.0);
        }

    "#;

    let shader = shader::Shader::new(vertex_shader_src, fragment_shader_src);


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
        game_window.win.get_size().0 as f32 / game_window.win.get_size().1 as f32,
        0.1,
        100.0,
    );

    // Test objects (will be removed later)
    #[allow(unused_mut)]
    let mut objects: Vec<Box<dyn Render>> = vec![
        Part::new(
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(45.0, 0.0, 0.0),
            glm::vec3(1.0, 1.0, 1.0),
            glm::vec3(1.0, 0.5, 0.31),
        ),
        Part::new(
            glm::vec3(2.0, 0.0, 0.0),
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(1.0, 1.0, 1.0),
            glm::vec3(1.0, 0.5, 0.31),
        ),
    ];

    // =============================================================
    // ========================= Main Loop =========================
    // =============================================================

    let mut mousehandler = MouseHandler::new(0., 0.);
    game_window.win.set_cursor_mode(glfw::CursorMode::Disabled);

    debug!("Starting main loop...");

    while !game_window.win.should_close() {
        // ----- DELTA TIME -----
        let delta_time = game_window.tick();

        game_window.glfw.poll_events();

        // ------------------------ Input -------------------------
        for (_, event) in glfw::flush_messages(&game_window.ev) {
            match event {
                glfw::WindowEvent::Key(key, _, action, _) => {
                    if (key as usize) < windowing::KEY_COUNT {
                        match action {
                            Action::Press => game_window.key_states[key as usize] = true,
                            Action::Release => game_window.key_states[key as usize] = false,
                            _ => {}
                        }
                    }

                    if key == Key::Escape && action == Action::Press {
                        game_window.win.set_should_close(true);
                    }
                    if key == Key::LeftAlt && action == Action::Press {
                        mousehandler.locked = !mousehandler.locked;
                        if mousehandler.locked {
                            game_window.win.set_cursor_mode(glfw::CursorMode::Disabled); // Captures and hides cursor
                        } else {
                            game_window.win.set_cursor_mode(glfw::CursorMode::Normal); // Captures and hides cursor
                        }
                    }
                }
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
        let key_states = game_window.key_states;

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

        // ---------------------- User Script ----------------------
        // (*objects[0]).rotate(glm::vec3(0.0, 20.0 * delta_time, 0.0));

        // ----------------------- Rendering -----------------------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            shader.use_program();

            let view = camera.get_view_matrix();
            let projection = camera.get_projection_matrix();

            shader.set_mat4("view", &view).unwrap();
            shader.set_mat4("projection", &projection).unwrap();

            for obj in &objects {
                obj.render(&shader);
            }
        }

        game_window.win.swap_buffers();
    }
    debug!("Closed")
}
