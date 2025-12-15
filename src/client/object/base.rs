use std::any::Any;

pub trait Render : Any {
    fn render(&self, shader: &crate::shader::Shader);

    fn as_any(&self) -> &dyn Any;
}
