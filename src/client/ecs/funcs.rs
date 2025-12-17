use crate::Part;
use crate::ecs::ecs::{self as ECS};
use crate::graphics::shader::Shader;
use crate::graphics::texture::Texture;
use nalgebra_glm as glm;

pub fn spawn_part(
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
