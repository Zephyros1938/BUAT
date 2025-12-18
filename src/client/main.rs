use gl;
use glfw::{Action, Context, Key};
use log::debug;
use log4rs;
use mini_redis::client;
use nalgebra_glm::{self as glm, Vec3};

mod ecs;
mod graphics;
mod input;
mod object;
mod util;

use crate::{
    ecs::{
        ecs::{self as ECS, Position},
        funcs::{add_render_data_to_world, spawn_line, spawn_part},
    },
    graphics::{
        camera::{self, Camera3d},
        shader, texture,
        windowing::{self, GameWindow, GameWindowHints},
    },
    input::mousehandler::MouseHandler,
    object::{mesh::obj_loader::load_obj_to_render_data, part::Part},
};

// ======================== Server Connection ========================
async fn preconnect(uri: &str) {
    match client::connect(uri).await {
        Ok(mut client) => {
            let result = client.get("ping").await;
            println!("Server pinged. Result: {:?}", result);
        }
        Err(_) => {
            println!("Can't connect to server!");
            std::process::exit(1);
        }
    }
}

// ============================ Main Program =========================
#[tokio::main]
async fn main() {
    // --------------------------- Logging ----------------------------
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    debug!("Launched BUAT v{}", env!("CARGO_PKG_VERSION"));

    // ------------------------ Connect to Server ---------------------
    let args: Vec<String> = std::env::args().collect();
    debug!("Client args: {:?}", args);

    let uri = args.get(2).map(|s| s.as_str()).unwrap_or("127.0.0.1:6700");
    let _sid: u64 = args
        .get(1)
        .expect("No ID specified!")
        .parse()
        .expect("Invalid ID!");

    preconnect(uri).await;

    // --------------------------- Window -----------------------------
    let mut game_window = GameWindow::new(GameWindowHints {
        gl_context: (3, 3),
        profile: glfw::OpenGlProfileHint::Core,
        title: &format!("BUAT {}", env!("CARGO_PKG_VERSION")),
        fullscreen: false,
        size: (1080, 720),
    })
    .unwrap();

    // ---------------------------- GL Setup --------------------------
    gl::load_with(|s| {
        game_window
            .win
            .get_proc_address(s)
            .map(|p| p as *const _)
            .unwrap_or(std::ptr::null())
    });
    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);
        gl::CullFace(gl::BACK);
        gl::FrontFace(gl::CCW);
    }

    // ---------------------------- Shaders ---------------------------
    let shader_norm = shader::Shader::from_files(
        "assets/shaders/part_default.vert",
        "assets/shaders/part_default.frag",
    )
    .unwrap();
    let shader_tex = shader::Shader::from_files(
        "assets/shaders/part_tex.vert",
        "assets/shaders/part_tex.frag",
    )
    .unwrap();
    let shader_mesh = shader::Shader::from_files(
        "assets/shaders/mesh_default.vert",
        "assets/shaders/mesh_default.frag",
    )
    .unwrap();
    let shader_line =
        shader::Shader::from_files("assets/shaders/line.vert", "assets/shaders/line.frag").unwrap();
    let texture_test =
        texture::load_texture_from_file("assets/material/wood.png", Default::default()).unwrap();

    // --------------------------- Camera -----------------------------
    let (width, height) = game_window.win.get_size();
    let mut camera = Camera3d::new(
        glm::vec3(0., 0., 0.),
        glm::vec3(0., 1., 0.),
        -0.0,
        0.0,
        0.15,
        1.5,
        45.0,
        true,
        width as f32 / height as f32,
        0.1,
        100.0,
    );

    // ---------------------------- ECS Setup -------------------------
    let mut world = ECS::World::new();

    spawn_part(
        &mut world,
        glm::vec3(0., 0., 0.),
        glm::vec3(0., 0., 0.),
        glm::vec3(1., 1., 1.),
        glm::vec3(1., 1., 1.),
        &shader_norm,
        None,
    );

    for obj in &["assets/rising_sun.obj", "assets/voidstar.obj"] {
        let position = if *obj == "assets/voidstar.obj" {
            glm::vec3(0., 0., 5.)
        } else {
            glm::vec3(0., 0., 0.)
        };
        let color = if *obj == "assets/voidstar.obj" {
            glm::vec3(0., 1., 0.)
        } else {
            glm::vec3(1., 0., 0.)
        };
        for render_data in load_obj_to_render_data(obj, true, true).unwrap() {
            add_render_data_to_world(
                &mut world,
                position,
                glm::vec3(0., 0., 0.),
                glm::vec3(1., 1., 1.),
                color,
                &render_data,
                &shader_mesh,
                None,
            );
        }
    }

    // ------------------------- Mouse Handler ------------------------
    let mut mousehandler = MouseHandler::new(0., 0.);
    game_window.win.set_cursor_mode(glfw::CursorMode::Disabled);

    // ------------------------- Spawn Axis Lines ---------------------
    let axes = [
        (
            Vec3::new(0., 0., 0.),
            Vec3::new(2., 0., 0.),
            Vec3::new(1., 0., 0.),
        ),
        (
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 2., 0.),
            Vec3::new(0., 1., 0.),
        ),
        (
            Vec3::new(0., 0., 0.),
            Vec3::new(0., 0., 2.),
            Vec3::new(0., 0., 1.),
        ),
    ];
    for (start, end, color) in axes {
        spawn_line(&mut world, start, end, color, &shader_line);
    }

    // ------------------------- Main Loop ----------------------------
    debug!("Starting main loop...");
    let mut total_time = 0.0;

    while !game_window.win.should_close() {
        let delta_time = game_window.tick();
        total_time += delta_time;
        game_window.glfw.poll_events();

        // ------------------------- Input -----------------------------
        for (_, event) in glfw::flush_messages(&game_window.ev) {
            match event {
                glfw::WindowEvent::Key(key, _, action, _)
                    if (key as usize) < windowing::KEY_COUNT =>
                {
                    game_window.key_states[key as usize] = matches!(action, Action::Press);

                    if key == Key::Escape && action == Action::Press {
                        game_window.win.set_should_close(true);
                    }
                    if key == Key::LeftAlt && action == Action::Press {
                        mousehandler.locked = !mousehandler.locked;
                        let mode = if mousehandler.locked {
                            glfw::CursorMode::Disabled
                        } else {
                            glfw::CursorMode::Normal
                        };
                        game_window.win.set_cursor_mode(mode);
                    }
                }
                glfw::WindowEvent::Size(width, height)
                | glfw::WindowEvent::FramebufferSize(width, height) => {
                    unsafe { gl::Viewport(0, 0, width, height) };
                    camera.aspect_ratio = width as f32 / height as f32;
                }
                glfw::WindowEvent::CursorPos(x, y) => {
                    let (dx, dy) = mousehandler.handle_mouse(x as f32, y as f32);
                    if mousehandler.locked {
                        camera.process_mouse(dx, dy);
                    }
                }
                glfw::WindowEvent::Scroll(_, yoffset) if mousehandler.locked => {
                    camera.process_scroll(yoffset as f32);
                }
                glfw::WindowEvent::MouseButton(_, Action::Press, _) if mousehandler.locked => {
                    spawn_line(
                        &mut world,
                        camera.position,
                        camera.position + camera.front * 100.0,
                        glm::vec3(rand::random(), rand::random(), rand::random()),
                        &shader_line,
                    );
                }
                _ => {}
            }
        }

        camera::debug_camera_movement(&mut camera, &game_window, delta_time);

        // ------------------------- Rendering --------------------------
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }

        let view = camera.get_view_matrix();
        let projection = camera.get_projection_matrix();

        world.positions.insert(
            0,
            Position(
                camera.position + Vec3::new(25. * total_time.sin(), 0., 25. * total_time.cos()),
            ),
        );

        for (&entity, render_data) in &world.part_render_data {
            if let Some(shader_ref) = world.shaders.get(&entity) {
                let shader = &shader_ref.0;
                let pos = world.positions.get(&entity).unwrap();
                let rot = world.rotations.get(&entity).unwrap();
                let scale = world.scales.get(&entity).unwrap();

                let rotation_matrix = glm::rotation(rot.0.y, &glm::vec3(0., 1., 0.))
                    * glm::rotation(rot.0.x, &glm::vec3(1., 0., 0.))
                    * glm::rotation(rot.0.z, &glm::vec3(0., 0., 1.));

                let model = glm::translate(&glm::identity(), &pos.0)
                    * rotation_matrix
                    * glm::scaling(&scale.0);

                unsafe {
                    gl::UseProgram(render_data.program_id);
                }
                shader.use_program();
                shader.set_mat4("view", &view).unwrap();
                shader.set_mat4("projection", &projection).unwrap();

                match world.entity_types.get(&entity) {
                    Some(ECS::EntityType::Line(color)) => {
                        shader.set_mat4("model", &glm::identity()).unwrap();
                        shader.set_vec3("uColor", color).unwrap();
                        unsafe {
                            gl::BindVertexArray(render_data.vao_id);
                            gl::DrawArrays(gl::LINES, 0, 2);
                            gl::BindVertexArray(0);
                        }
                    }
                    Some(ECS::EntityType::Part) => {
                        shader.set_mat4("model", &model).unwrap();
                        shader
                            .set_vec3("uColor", &world.colors.get(&entity).unwrap().0)
                            .unwrap();
                        shader.set_vec3("viewPos", &camera.position).unwrap();
                        shader
                            .set_vec3("lightPos", &world.positions.get(&0).unwrap().0)
                            .unwrap();
                        shader
                            .set_vec3("lightColor", &world.colors.get(&0).unwrap().0)
                            .unwrap();

                        if let Some(tex) = world.textures.get(&entity) {
                            tex.bind(0);
                        }

                        unsafe {
                            gl::BindVertexArray(render_data.vao_id);
                            gl::DrawElements(
                                gl::TRIANGLES,
                                render_data.index_count,
                                gl::UNSIGNED_INT,
                                std::ptr::null(),
                            );
                        }
                    }
                    _ => {}
                }
            }
        }

        game_window.win.swap_buffers();
    }

    debug!("Closed");
}
