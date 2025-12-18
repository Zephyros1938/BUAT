use crate::Part;
use crate::ecs::ecs::{self as ECS, PartRenderData};
use crate::graphics::shader::{Shader, VertexArrayObject};
use crate::graphics::texture::{self, Texture};
use crate::object::part::RenderData;
use crate::object::part::consts::{PART_INDICES_COLOR, PART_VERTICES};
use nalgebra_glm as glm;

// pub fn spawn_part(
//     world: &mut ECS::World,
//     position: glm::Vec3,
//     rotation: glm::Vec3,
//     scale: glm::Vec3,
//     color: glm::Vec3,
//     shader: &Shader,
//     texture: Option<Texture>,
// ) -> ECS::Entity {
//     let entity = world.create_entity();

//     let part = match texture {
//         Some(tex) => Part::gen_part_textured(position, rotation, scale, color, tex, shader),
//         None => Part::new(position, rotation, scale, color, shader),
//     };

//     world.positions.insert(entity, ECS::Position(position));
//     world.rotations.insert(entity, ECS::Rotation(rotation));
//     world.scales.insert(entity, ECS::Scale(scale));
//     world.colors.insert(entity, ECS::Color(color));
//     world.part_render_data.insert(
//         entity,
//         ECS::PartRenderData {
//             program_id: part.render_data.program_id,
//             vao_id: part.render_data.vao.id,
//             index_count: part.render_data.index_count,
//         },
//     );
//     world.shaders.insert(entity, ECS::Shader(*shader));

//     if let Some(tex) = part.texture {
//         world.textures.insert(entity, tex);
//     }

//     entity
// }

pub fn spawn_part(
    world: &mut ECS::World,
    position: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,
    color: glm::Vec3,
    shader: &Shader,
    texture: Option<Texture>,
) -> ECS::Entity {
    let entity = world.create_entity();

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
            (PART_VERTICES.len() * std::mem::size_of::<f32>()) as _,
            PART_VERTICES.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // -------- EBO --------
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (PART_INDICES_COLOR.len() * std::mem::size_of::<u32>()) as _,
            PART_INDICES_COLOR.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        // -------- Vertex Attributes --------
        let stride = 3 * std::mem::size_of::<f32>() as i32;

        // Position (location = 0)
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
        gl::EnableVertexAttribArray(0);

        gl::BindVertexArray(0);
    }

    world.positions.insert(entity, ECS::Position(position));
    world.rotations.insert(entity, ECS::Rotation(rotation));
    world.scales.insert(entity, ECS::Scale(scale));
    world.colors.insert(entity, ECS::Color(color));
    world.part_render_data.insert(
        entity,
        ECS::PartRenderData {
            program_id: shader.program,
            vao_id: vao.id,
            index_count: PART_INDICES_COLOR.len() as i32,
        },
    );
    world.shaders.insert(entity, ECS::Shader(*shader));
    world.entity_types.insert(entity, ECS::EntityType::Part);

    if let Some(tex) = texture {
        world.textures.insert(entity, tex);
    }

    entity
}

pub fn add_render_data_to_world(
    world: &mut ECS::World,
    position: glm::Vec3,
    rotation: glm::Vec3,
    scale: glm::Vec3,
    color: glm::Vec3,
    render_data: &RenderData,
    shader: &Shader,
    texture: Option<Texture>,
) {
    let entity = world.create_entity();

    world.positions.insert(entity, ECS::Position(position));
    world.rotations.insert(entity, ECS::Rotation(rotation));
    world.scales.insert(entity, ECS::Scale(scale));
    world.colors.insert(entity, ECS::Color(color));
    world.shaders.insert(entity, ECS::Shader(*shader));
    if let Some(tex) = texture {
        world.textures.insert(entity, tex);
    }
    world.part_render_data.insert(
        entity,
        PartRenderData {
            program_id: shader.program,
            vao_id: render_data.vao.id,
            index_count: render_data.index_count,
        },
    );
    world.entity_types.insert(entity, ECS::EntityType::Part);
}

pub fn spawn_line(
    world: &mut ECS::World,
    start: glm::Vec3,
    end: glm::Vec3,
    color: glm::Vec3,
    shader: &Shader,
) -> ECS::Entity {
    let entity = world.create_entity();

    let mut vao = 0;
    let mut vbo = 0;
    let line_vertices: [f32; 6] = [start.x, start.y, start.z, end.x, end.y, end.z];

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (line_vertices.len() * std::mem::size_of::<f32>()) as isize,
            line_vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            std::ptr::null(),
        );
        gl::EnableVertexAttribArray(0);
        gl::BindVertexArray(0);
    }

    world.positions.insert(entity, ECS::Position(start));
    world
        .rotations
        .insert(entity, ECS::Rotation(glm::vec3(0., 0., 0.)));
    world
        .scales
        .insert(entity, ECS::Scale(glm::vec3(1., 1., 1.)));

    world
        .entity_types
        .insert(entity, ECS::EntityType::Line(start, end, color));
    world.shaders.insert(entity, ECS::Shader(*shader));
    world.part_render_data.insert(
        entity,
        ECS::PartRenderData {
            program_id: shader.program,
            vao_id: vao,
            index_count: 2,
        },
    );

    entity
}
