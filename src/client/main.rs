use glfw::{Action, Context, Key};
use log::{debug, info};
use log4rs;
use mini_redis::client;
use nalgebra_glm::{self as glm, vec3};

mod ecs;
mod graphics;
mod input;
mod object;
mod util;

use crate::{
    ecs::ecs as ECS,
    graphics::{
        camera::{self, Camera3d},
        shader::{self, Shader},
        texture::{self, Texture},
        windowing::{self, GameWindow, GameWindowHints},
    },
    input::mousehandler::MouseHandler,
    object::{
        mesh_loader::{self, MESH_UV, MESH_VERT},
        part::Part,
    },
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
        std::process::exit(1);
    }
}

// =============================================================
// ========================  Functions  ========================
// =============================================================

fn spawn_part(
    world: &mut ECS::World,
    position: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,
    color: glm::Vec3,
    shader: &Shader,
    texture: Option<Texture>,
) -> ECS::Entity {
    let entity = world.next_entity_id as ECS::Entity;
    world.next_entity_id += 1;

    let part = match texture {
        Some(tex) => Part::gen_part_textured(position, rotation, scale, color, tex, shader),
        None => Part::new(position, rotation, scale, color, shader),
    };

    world.positions.insert(entity, ECS::Position(position));
    world.rotations.insert(entity, ECS::Rotation(rotation));
    world.scales.insert(entity, ECS::Scale(scale));
    world.colors.insert(entity, ECS::Color(color));
    world.part_render_data.insert(
        entity,
        ECS::PartRenderData {
            program_id: part.render_data.program_id,
            vao_id: part.render_data.vao.id,
            index_count: part.render_data.index_count,
        },
    );
    world.shaders.insert(entity, ECS::Shader(*shader));

    if let Some(tex) = part.texture {
        world.textures.insert(entity, tex);
    }

    entity
}

// =============================================================
// ======================= Main Program ========================
// =============================================================

#[tokio::main]
async fn main() {
    // ------------------------ Logging -------------------------
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let _x = mesh_loader::load_vertices_from_obj("assets/test.obj", MESH_VERT | MESH_UV).unwrap();
    info!("{:?}", _x);

    debug!(
        "{}",
        format!("Launched BUAT v{}", env!("CARGO_PKG_VERSION"))
    );

    // -------------------- Connect to Server --------------------
    let env_args: Vec<String> = std::env::args().collect();
    debug!("Client args: {:?}", env_args);

    let uri_default: String = String::from("127.0.0.1:6700");
    let uri: String = env_args.get(2).unwrap_or(&uri_default).to_string();

    let _sid: u64 = env_args
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
    let texture_test = texture::load_texture_from_file(
        "assets/material/wood.png",
        texture::TextureLoadOptions {
            ..Default::default()
        },
    )
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
        game_window.win.get_size().0 as f32 / game_window.win.get_size().1 as f32,
        0.1,
        100.0,
    );

    // ------------------------- Initialize ECS -------------------------
    let mut world = ECS::World::new();

    spawn_part(
        &mut world,
        glm::vec3(0., 0., 0.),
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(5.0, 5.0, 5.0),
        glm::vec3(1., 0., 0.),
        &shader_tex,
        Some(texture_test),
    );

    // =============================================================
    // ========================= Main Loop =========================
    // =============================================================

    let mut mousehandler = MouseHandler::new(0., 0.);
    game_window.win.set_cursor_mode(glfw::CursorMode::Disabled);

    debug!("Starting main loop...");

    // ----------------------- Main Loop -----------------------
    let mut total_time = 0.0;
    while !game_window.win.should_close() {
        let delta_time = game_window.tick();
        total_time += delta_time;
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
                glfw::WindowEvent::Scroll(xpos, ypos) => {
                    if mousehandler.locked {
                        camera.process_scroll(ypos as f32);
                    }
                }

                _ => {}
            }
        }

        camera::debug_camera_movement(&mut camera, &game_window, delta_time);

        // ----------------------- ECS Update Systems -----------------------
        // test rotation system
        for (&_, rotation) in &mut world.rotations {
            rotation.0 += glm::vec3(0.25, 0.7, 0.33) * delta_time;
        }

        // ----------------------- Rendering System -----------------------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);

            let view = camera.get_view_matrix();
            let projection = camera.get_projection_matrix();

            for (&entity, render_data) in &world.part_render_data {
                // Determine shader based on entity
                if let Some(c) = world.shaders.get(&entity) {
                    let target_shader = c.0;

                    // Matrix construction
                    let pos = world.positions.get(&entity).unwrap();
                    let rot = world.rotations.get(&entity).unwrap();
                    let scale = world.scales.get(&entity).unwrap();
                    let rot_x = glm::rotation(rot.0.x, &glm::Vec3::new(1.0, 0.0, 0.0));
                    let rot_y = glm::rotation(rot.0.y, &glm::Vec3::new(0.0, 1.0, 0.0));
                    let rot_z = glm::rotation(rot.0.z, &glm::Vec3::new(0.0, 0.0, 1.0));
                    let rotation_matrix = rot_y * rot_x * rot_z;
                    let model = glm::translate(&glm::identity(), &pos.0)
                        * rotation_matrix
                        * glm::scaling(&scale.0);

                    // GL calls
                    gl::UseProgram(render_data.program_id);
                    target_shader.use_program();
                    target_shader.set_mat4("view", &view).unwrap();
                    target_shader.set_mat4("projection", &projection).unwrap();
                    target_shader.set_mat4("model", &model).unwrap();
                    target_shader
                        .set_vec3("uColor", &world.colors.get(&entity).unwrap().0)
                        .unwrap();
                    (world.colors.get_mut(&entity).unwrap()).0 = glm::Vec3::new(
                        total_time % 1.0,
                        (total_time + 0.33) % 1.0,
                        (total_time + 0.66) % 1.0,
                    );

                    if let Some(t) = world.textures.get(&entity) {
                        // If there's a texture then apply it
                        t.bind(0);
                    }
                    gl::BindVertexArray(render_data.vao_id);
                    gl::DrawElements(
                        gl::TRIANGLES,
                        render_data.index_count,
                        gl::UNSIGNED_INT,
                        std::ptr::null(),
                    );
                }
            }
        }

        game_window.win.swap_buffers();
    }
    debug!("Closed")
}
