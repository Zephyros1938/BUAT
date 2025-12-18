use std::collections::HashMap;

use nalgebra_glm::{self as glm, Vec3};

use crate::graphics::{shader, texture::Texture};

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

pub enum EntityType {
    Part,
    Special,
    Line(Vec3), // start, end, color
}

pub struct World {
    pub next_entity_id: usize,
    pub positions: HashMap<usize, Position>,
    pub rotations: HashMap<usize, Rotation>,
    pub scales: HashMap<usize, Scale>,
    pub velocities: HashMap<usize, Velocity>,
    pub colors: HashMap<usize, Color>,
    pub part_render_data: HashMap<usize, PartRenderData>,
    pub textures: HashMap<usize, Texture>,
    pub shaders: HashMap<usize, Shader>,
    pub entity_types: HashMap<usize, EntityType>,
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

    pub fn create_entity(&mut self) -> usize {
        let entity_id = self.next_entity_id;
        self.next_entity_id += 1;
        entity_id
    }

    pub fn destroy_entity(&mut self, entity: usize) {
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
