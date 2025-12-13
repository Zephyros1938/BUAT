use gl::{self};
use nalgebra_glm as glm;

use crate::Render;
use crate::shader::VertexArrayObject;

pub struct RenderData {
    pub vao: VertexArrayObject,
    pub index_count: i32,
}

pub struct Part {
    pos: glm::Vec3,
    rot: glm::Vec3,
    size: glm::Vec3,
    color: glm::Vec3,
    render_data: RenderData,
}

impl Part {
    pub fn new(pos: glm::Vec3, rot: glm::Vec3, size: glm::Vec3, color: glm::Vec3) -> Box<Part> {
        let vertices: [f32; 48] = [
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
        ];

        let indices: [u32; 3 * 2 * 6] = [
            0, 3, 1, 0, 2, 3, // back
            4, 5, 7, 4, 7, 6, // front
            4, 2, 0, 4, 6, 2, // left
            1, 3, 7, 1, 7, 5, // right
            2, 6, 7, 2, 7, 3, // top
            0, 1, 5, 0, 5, 4, // bottom
        ];

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
                (vertices.len() * std::mem::size_of::<f32>()) as _,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // -------- EBO --------
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as _,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // -------- Vertex Attributes --------
            let stride = 6 * std::mem::size_of::<f32>() as i32;

            // Position (location = 0)
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            // Color (location = 1)
            gl::VertexAttribPointer(
                1,
                3,
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
            color,
            render_data: RenderData {
                vao,
                index_count: indices.len() as i32,
            },
        })
    }

    pub fn get_model_matrix(&self) -> glm::Mat4 {
        let mut model = glm::identity();
        model = glm::translate(&model, &self.pos);
        model = glm::rotate(&model, self.rot.x.to_radians(), &glm::vec3(1.0, 0.0, 0.0));
        model = glm::rotate(&model, self.rot.y.to_radians(), &glm::vec3(0.0, 1.0, 0.0));
        model = glm::rotate(&model, self.rot.z.to_radians(), &glm::vec3(0.0, 0.0, 1.0));
        model = glm::scale(&model, &self.size);
        model
    }

    #[allow(dead_code)]
    pub fn translate(&mut self, translation: glm::Vec3) {
        self.pos += translation;
    }
    #[allow(dead_code)]
    pub fn rotate(&mut self, rotation: glm::Vec3) {
        self.rot += rotation;
    }
    #[allow(dead_code)]
    pub fn scale(&mut self, scale: glm::Vec3) {
        self.size += scale;
    }
}

impl Render for Part {
    fn render(&self, shader: &crate::shader::Shader) {
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
}
