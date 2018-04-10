#[derive(Default)]
pub struct Chip8 {}

impl Chip8 {
    pub fn run(&mut self, rom_path: &String) {
        println!("{}", rom_path)
    }
}
