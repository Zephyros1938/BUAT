use gl::{self};

use crate::shader::VertexArrayObject;

pub trait Render {
    fn render(
        &self,
        shader: &crate::shader::Shader
    );
}
