pub trait Renderer {
    fn render(&mut self, display: &[bool], width: &usize, height: &usize);
}
