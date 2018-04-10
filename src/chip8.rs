const RAM_SIZE: usize = 4 * 1024;
const REGISTER_SIZE: usize = 16;
const STACK_SIZE: usize = 24;
const KEYBOARD_SIZE: usize = 16;
const DISPLAY_SIZE: usize = 64 * 32;

pub struct Chip8 {
    ram: [u8; RAM_SIZE],
    registers: [u8; REGISTER_SIZE],
    address_register: u16,
    stack: [u16; STACK_SIZE],
    timer_delay: u8,
    timer_sound: u8,
    keyboard: [bool; KEYBOARD_SIZE],
    display: [bool; DISPLAY_SIZE],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            ram: [0; RAM_SIZE],
            registers: [0; REGISTER_SIZE],
            address_register: 0,
            stack: [0; STACK_SIZE],
            timer_delay: 0,
            timer_sound: 0,
            keyboard: [false; KEYBOARD_SIZE],
            display: [false; DISPLAY_SIZE],
        }
    }

    pub fn run(&mut self, rom_path: &String) {
        println!("{}", rom_path)
    }
}
