use std::collections::HashMap;

use nalgebra_glm::{self as glm, Vec3};

use crate::graphics::{shader, texture::Texture};
pub type Entity = usize;

// Components
#[derive(Debug, Clone, Copy)]
pub struct Position(pub glm::Vec3);
#[derive(Debug, Clone, Copy)]
pub struct Rotation(pub glm::Vec3);
#[derive(Debug, Clone, Copy)]
pub struct Scale(pub glm::Vec3);
#[derive(Debug, Clone, Copy)]
pub struct Velocity(pub glm::Vec3);
#[derive(Debug, Clone, Copy)]
pub struct Color(pub glm::Vec3);
#[derive(Debug, Clone, Copy)]
pub struct PartRenderData {
    pub program_id: u32,
    pub vao_id: u32,
    pub index_count: i32,
}
#[derive(Debug, Clone, Copy)]
pub struct Shader(pub shader::Shader);
// World

pub enum EntityType {
    Part,
    Special,
    Line(Vec3, Vec3, Vec3), // start, end, color
}

pub struct World {
    pub next_entity_id: usize,
    pub positions: HashMap<Entity, Position>,
    pub rotations: HashMap<Entity, Rotation>,
    pub scales: HashMap<Entity, Scale>,
    pub velocities: HashMap<Entity, Velocity>,
    pub colors: HashMap<Entity, Color>,
    pub part_render_data: HashMap<Entity, PartRenderData>,
    pub textures: HashMap<Entity, Texture>,
    pub shaders: HashMap<Entity, Shader>,
    pub entity_types: HashMap<Entity, EntityType>,
}



impl World {
    pub fn new() -> Self {
        World {
            next_entity_id: 0,
            positions: HashMap::new(),
            rotations: HashMap::new(),
            scales: HashMap::new(),
            velocities: HashMap::new(),
            colors: HashMap::new(),
            part_render_data: HashMap::new(),
            textures: HashMap::new(),
            shaders: HashMap::new(),
            entity_types: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        entity_id
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.positions.remove(&entity);
        self.rotations.remove(&entity);
        self.scales.remove(&entity);
        self.velocities.remove(&entity);
        self.colors.remove(&entity);
        self.part_render_data.remove(&entity);
        self.textures.remove(&entity);
        self.shaders.remove(&entity);
        self.entity_types.remove(&entity);
    }
}
