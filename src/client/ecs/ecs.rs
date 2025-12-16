use std::collections::HashMap;

use nalgebra_glm as glm;

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

pub struct World {
    pub next_entity_id: u64,
    pub positions: HashMap<Entity, Position>,
    pub rotations: HashMap<Entity, Rotation>,
    pub scales: HashMap<Entity, Scale>,
    pub velocities: HashMap<Entity, Velocity>,
    pub colors: HashMap<Entity, Color>,
    pub part_render_data: HashMap<Entity, PartRenderData>,
    pub textures: HashMap<Entity, Texture>,
    pub shaders: HashMap<Entity, Shader>
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
            shaders: HashMap::new()
        }
    }
}
