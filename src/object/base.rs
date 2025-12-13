pub trait Render {
    fn render(&self, shader: &crate::shader::Shader);
}
