use gl::{self};
use nalgebra_glm as glm;

use crate::shader::VertexArrayObject;

pub trait Render {
    fn render(
        &self,
        shader: &crate::shader::Shader,
        view_matrix: &glm::Mat4,
        projection_matrix: &glm::Mat4,
    );
}
