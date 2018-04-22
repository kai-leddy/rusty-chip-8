pub mod cli;

use super::chip8::display::Display;

pub trait Renderable {
    fn render(&mut self, display: &Display);
}
