use std::any::Any;

pub trait Render {
    fn render(&self, shader: &crate::shader::Shader);

    fn as_any(&self) -> &dyn Any;

    fn as_mut_any(&mut self) -> &mut dyn Any;
}
