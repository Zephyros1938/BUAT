use gl::{self};
use nalgebra_glm as glm;

use crate::shader::VertexArrayObject;

pub struct RenderData {
    pub vao: VertexArrayObject,
    pub index_count: i32,
}

pub struct BasePart {
    pos: glm::Vec3,
    rot: glm::Vec3,
    size: glm::Vec3,
    color: glm::Vec3,
    render_data: RenderData,
}

impl BasePart {
    pub fn new(pos: glm::Vec3, rot: glm::Vec3, size: glm::Vec3, color: glm::Vec3) -> BasePart {
        let cube_data: [f32; 48] = [
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

        let indexes: [u32; 3 * 2] = [
            0, 3, 2, 0, 1, 2,
        ];

        BasePart {
            pos,
            rot,
            size,
            color,
        }
    }

    fn render(
        &self,
        shader: &crate::shader::Shader,
        view_matrix: &glm::Mat4,
        projection_matrix: &glm::Mat4,
    ) {
        let modelMatrix = self.get_model_matrix();
        shader.set_mat4("model", &modelMatrix).unwrap();
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
    fn get_model_matrix(&self) -> glm::Mat4 {
        let mut model = glm::identity();
        model = glm::translate(&model, &self.pos);
        model = glm::rotate(&model, self.rot.x, &glm::vec3(1.0, 0.0, 0.0));
        model = glm::rotate(&model, self.rot.y, &glm::vec3(0.0, 1.0, 0.0));
        model = glm::rotate(&model, self.rot.z, &glm::vec3(0.0, 0.0, 1.0));
        model = glm::scale(&model, &self.size);
        model
    }

    fn translate(&mut self, translation: glm::Vec3) {
        self.pos += translation;
    }

    fn rotate(&mut self, rotation: glm::Vec3) {
        self.rot += rotation;
    }

    fn scale(&mut self, scale: glm::Vec3) {
        self.size += scale;
    }
}
