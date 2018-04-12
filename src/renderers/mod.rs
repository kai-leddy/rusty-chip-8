pub mod cli;

pub trait Renderable {
    fn render(&mut self, display: &[bool], width: &usize, height: &usize);
}
