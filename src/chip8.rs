use std::fs::File;
use std::io::Read;

use super::renderers::Renderable;

const RAM_SIZE: usize = 4 * 1024;
const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 24;
const KEYBOARD_SIZE: usize = 16;
const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Chip8 {
    renderer: Box<Renderable>,
    ram: [u8; RAM_SIZE],
    registers: [u8; REGISTER_SIZE],
    address_register: u16,
    program_counter: u16,
    stack: [u16; STACK_SIZE],
    stack_pointer: u8,
    timer_delay: u8,
    timer_sound: u8,
    keyboard: [bool; KEYBOARD_SIZE],
    display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

impl Chip8 {
    pub fn new(renderer: Box<Renderable>) -> Chip8 {
        Chip8 {
            renderer: renderer,
            ram: [0; RAM_SIZE],
            registers: [0; REGISTER_SIZE],
            address_register: 0,
            program_counter: 0,
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
            timer_delay: 0,
            timer_sound: 0,
            keyboard: [false; KEYBOARD_SIZE],
            display: [true; DISPLAY_WIDTH * DISPLAY_HEIGHT],
        }
    }

    pub fn run(&mut self, rom_path: &String) {
        self.load(rom_path);
        self.renderer
            .render(&self.display, &DISPLAY_WIDTH, &DISPLAY_HEIGHT);
    }

    fn load(&mut self, rom_path: &String) {
        File::open(rom_path).unwrap().read(&mut self.ram).unwrap();
    }
}
