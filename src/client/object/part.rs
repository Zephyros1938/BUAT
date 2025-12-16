#![allow(dead_code)]
use std::any::Any;

use gl::{self};
use nalgebra_glm as glm;

use crate::Render;
use crate::graphics::shader::Shader;
use crate::graphics::texture::Texture;
use crate::shader::VertexArrayObject;

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

fn gen_part_vertices_color(color: glm::Vec3) -> [f32; 48] {
    [
        // Back face (z = -0.5)
        -0.5, -0.5, -0.5, color.x, color.y, color.z, // left,  bottom, back
        0.5, -0.5, -0.5, color.x, color.y, color.z, // right, bottom, back
        -0.5, 0.5, -0.5, color.x, color.y, color.z, // left,  top,    back
        0.5, 0.5, -0.5, color.x, color.y, color.z, // right, top,    back
        // Front face (z = +0.5)
        -0.5, -0.5, 0.5, color.x, color.y, color.z, // left,  bottom, front
        0.5, -0.5, 0.5, color.x, color.y, color.z, // right, bottom, front
        -0.5, 0.5, 0.5, color.x, color.y, color.z, // left,  top,    front
        0.5, 0.5, 0.5, color.x, color.y, color.z, // right, top,    front
    ]
}

fn gen_inverse_part_vertices_color(color: glm::Vec3) -> [f32; 48] {
    [
        -0.5, -0.5, -0.5, color.x, color.y, color.z, // left,  bottom, back
        00.5, -0.5, -0.5, color.x, color.y, color.z, // right, bottom, back
        -0.5, -0.5, 00.5, color.x, color.y, color.z, // left,  top,    back
        00.5, -0.5, 00.5, color.x, color.y, color.z, // right, top,    back
        -0.5, 00.5, -0.5, color.x, color.y, color.z, // left,  bottom, front
        00.5, 00.5, -0.5, color.x, color.y, color.z, // right, bottom, front
        -0.5, 00.5, 00.5, color.x, color.y, color.z, // left,  top,    front
        00.5, 00.5, 00.5, color.x, color.y, color.z, // right, top,    front
    ]
}

const PART_VERTICES: [f32; 8 * 3] = [
    // Back face (z = -0.5)
    -0.5, -0.5, -0.5, // left,  bottom, back
    0.5, -0.5, -0.5, // right, bottom, back
    -0.5, 0.5, -0.5, // left,  top,    back
    0.5, 0.5, -0.5, // right, top,    back
    // Front face (z = +0.5)
    -0.5, -0.5, 0.5, // left,  bottom, front
    0.5, -0.5, 0.5, // right, bottom, front
    -0.5, 0.5, 0.5, // left,  top,    front
    0.5, 0.5, 0.5, // right, top,    front
];

const PART_VERTICES_INVERSE: [f32; 8 * 3] = [
    // Back face (z = -0.5)
    -0.5, -0.5, -0.5, // left,  bottom, back
    0.5, -0.5, -0.5, // right, bottom, back
    -0.5, -0.5, 0.5, // left,  top,    back
    0.5, -0.5, 0.5, // right, top,    back
    // Front face (z = +0.5)
    -0.5, 0.5, -0.5, // left,  bottom, front
    0.5, 0.5, -0.5, // right, bottom, front
    -0.5, 0.5, 0.5, // left,  top,    front
    0.5, 0.5, 0.5, // right, top,    front
];

const PART_INDICES_COLOR: [u32; 3 * 2 * 6] = [
    0, 3, 1, 0, 2, 3, // back
    4, 5, 7, 4, 7, 6, // front
    4, 2, 0, 4, 6, 2, // left
    1, 3, 7, 1, 7, 5, // right
    2, 6, 7, 2, 7, 3, // top
    0, 1, 5, 0, 5, 4, // bottom
];

const PART_VERTICES_TEX: [f32; 24 * 5] = [
    // Back
    -0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, -0.5, 1.0, 0.0, 0.5, 0.5, -0.5, 1.0, 1.0, -0.5, 0.5,
    -0.5, 0.0, 1.0, // Front
    -0.5, -0.5, 0.5, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5, 0.5, 0.5,
    0.0, 1.0, // Left
    -0.5, -0.5, 0.5, 0.0, 0.0, -0.5, -0.5, -0.5, 1.0, 0.0, -0.5, 0.5, -0.5, 1.0, 1.0, -0.5, 0.5,
    0.5, 0.0, 1.0, // Right
    0.5, -0.5, -0.5, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, 0.5, 0.5, -0.5,
    0.0, 1.0, // Top
    -0.5, 0.5, -0.5, 0.0, 0.0, 0.5, 0.5, -0.5, 1.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0, -0.5, 0.5, 0.5,
    0.0, 1.0, // Bottom
    -0.5, -0.5, 0.5, 0.0, 0.0, 0.5, -0.5, 0.5, 1.0, 0.0, 0.5, -0.5, -0.5, 1.0, 1.0, -0.5, -0.5,
    -0.5, 0.0, 1.0,
];
const PART_INDICES_TEX: [u32; 36] = [
    1, 0, 2, 2, 0, 3, // front
    4, 5, 6, 4, 6, 7, // back
    9, 8, 10, 10, 8, 11, // left
    13, 12, 14, 14, 12, 15, // right
    17, 16, 18, 18, 16, 19, // top
    21, 20, 22, 22, 20, 23, // bottom
];
const PART_INDICES_INVERT_TEX: [u32; 36] = [
    0, 1, 2, 0, 2, 3, // front
    5, 4, 6, 6, 4, 7, // back
    8, 9, 10, 8, 10, 11, // left
    12, 13, 14, 12, 14, 15, // right
    16, 17, 18, 16, 18, 19, // top
    20, 21, 22, 20, 22, 23, // bottom
];

pub trait PartImpl {
    fn get_model_matrix(&self) -> glm::Mat4 {
        glm::Mat4::identity()
    }
    fn translate(&mut self, translation: glm::Vec3);
    fn rotate(&mut self, rotation: glm::Vec3);
    fn scale(&mut self, scale: glm::Vec3);
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
                (PART_VERTICES_INVERSE.len() * std::mem::size_of::<f32>()) as _,
                PART_VERTICES_INVERSE.as_ptr() as *const _,
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

impl PartImpl for Part {
    fn get_model_matrix(&self) -> glm::Mat4 {
        let mut model = glm::identity();
        model = glm::translate(&model, &self.pos);
        model = glm::rotate(&model, self.rot.x.to_radians(), &glm::vec3(1.0, 0.0, 0.0));
        model = glm::rotate(&model, self.rot.y.to_radians(), &glm::vec3(0.0, 1.0, 0.0));
        model = glm::rotate(&model, self.rot.z.to_radians(), &glm::vec3(0.0, 0.0, 1.0));
        model = glm::scale(&model, &self.size);
        model
    }

    #[allow(dead_code)]
    fn translate(&mut self, translation: glm::Vec3) {
        self.pos += translation;
    }
    #[allow(dead_code)]
    fn rotate(&mut self, rotation: glm::Vec3) {
        self.rot += rotation;
    }
    #[allow(dead_code)]
    fn scale(&mut self, scale: glm::Vec3) {
        self.size += scale;
    }
}

impl Render for Part {
    fn render(&self, shader: &crate::shader::Shader) {
        shader.use_program();
        let model_matrix = self.get_model_matrix();
        shader.set_mat4("model", &model_matrix).unwrap();
        self.render_data.vao.bind();
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.render_data.index_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
        self.render_data.vao.unbind();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}
