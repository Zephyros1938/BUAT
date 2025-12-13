use gl::{self};
use nalgebra_glm as glm;

use crate::shader::VertexArrayObject;

pub trait Render {
    fn render(
        &self,
        shader: &crate::shader::Shader
    );
}
