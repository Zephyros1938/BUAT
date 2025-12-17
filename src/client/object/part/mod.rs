#![allow(dead_code)]
use std::any::Any;

use gl::{self};
use nalgebra_glm as glm;

// use crate::Render;
use crate::{graphics::shader::Shader, object::part::consts::PART_VERTICES};
use crate::graphics::texture::Texture;
use crate::shader::VertexArrayObject;

pub mod consts;

use consts::{PART_INDICES_COLOR, PART_VERTICES_INVERSE, PART_INDICES_TEX, PART_VERTICES_TEX};

pub struct RenderData {
    pub vao: VertexArrayObject,
    pub index_count: i32,
    pub program_id: u32,
}

pub struct Part {
    pos: glm::Vec3,
    rot: glm::Vec3,
    size: glm::Vec3,
    color: glm::Vec3,
    pub render_data: RenderData,
    pub texture: Option<Texture>,
}



impl Part {
    pub fn new(
        pos: glm::Vec3,
        rot: glm::Vec3,
        size: glm::Vec3,
        color: glm::Vec3,
        shader: &Shader,
    ) -> Box<Part> {

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

        Box::new(Part {
            pos,
            rot,
            size,
            color,
            render_data: RenderData {
                vao,
                index_count: PART_INDICES_COLOR.len() as i32,
                program_id: shader.program,
            },
            texture: None,
        })
    }

    pub fn gen_part_textured(
        pos: glm::Vec3,
        rot: glm::Vec3,
        size: glm::Vec3,
        basecolor: glm::Vec3,
        texture: Texture,
        shader: &Shader,
    ) -> Box<Part> {
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
                (PART_VERTICES_TEX.len() * std::mem::size_of::<f32>()) as _,
                PART_VERTICES_TEX.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // -------- EBO --------
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (PART_INDICES_TEX.len() * std::mem::size_of::<u32>()) as _,
                PART_INDICES_TEX.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // -------- Vertex Attributes --------
            let stride = 5 * std::mem::size_of::<f32>() as i32;

            // Position (location = 0)
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // Color (location = 1)
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                stride,
                (3 * std::mem::size_of::<f32>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }

        Box::new(Part {
            pos,
            rot,
            size,
            color: basecolor,
            render_data: RenderData {
                vao,
                index_count: PART_INDICES_TEX.len() as i32,
                program_id: shader.program,
            },
            texture: Some(texture),
        })
    }
}

// pub trait PartImpl {
//     fn get_model_matrix(&self) -> glm::Mat4 {
//         glm::Mat4::identity()
//     }
//     fn translate(&mut self, translation: glm::Vec3);
//     fn rotate(&mut self, rotation: glm::Vec3);
//     fn scale(&mut self, scale: glm::Vec3);
// }

// impl PartImpl for Part {
//     fn get_model_matrix(&self) -> glm::Mat4 {
//         let mut model = glm::identity();
//         model = glm::translate(&model, &self.pos);
//         model = glm::rotate(&model, self.rot.x.to_radians(), &glm::vec3(1.0, 0.0, 0.0));
//         model = glm::rotate(&model, self.rot.y.to_radians(), &glm::vec3(0.0, 1.0, 0.0));
//         model = glm::rotate(&model, self.rot.z.to_radians(), &glm::vec3(0.0, 0.0, 1.0));
//         model = glm::scale(&model, &self.size);
//         model
//     }

//     #[allow(dead_code)]
//     fn translate(&mut self, translation: glm::Vec3) {
//         self.pos += translation;
//     }
//     #[allow(dead_code)]
//     fn rotate(&mut self, rotation: glm::Vec3) {
//         self.rot += rotation;
//     }
//     #[allow(dead_code)]
//     fn scale(&mut self, scale: glm::Vec3) {
//         self.size += scale;
//     }
// }