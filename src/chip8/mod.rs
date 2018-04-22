pub mod display;
mod interpreter;

use std::fs::File;
use std::io::Read;
use std::io::Error;
use std::time::Duration;
use std::thread;

use super::config;
use super::renderers::Renderable;
use self::display::Display;

pub struct Chip8<'a> {
    display: Display,
    renderer: &'a mut Renderable,
    ram: [u8; config::RAM_SIZE],
    registers: [u8; config::REGISTER_SIZE],
    address_register: usize,
    program_counter: usize,
    stack: [u16; config::STACK_SIZE],
    stack_pointer: usize,
    timer_delay: u8,
    timer_sound: u8,
    keyboard: [bool; config::KEYBOARD_SIZE],
}

impl<'a> Chip8<'a> {
    pub fn new(renderer: &mut Renderable) -> Chip8 {
        Chip8 {
            display: Display::new(),
            renderer: renderer,
            ram: [0; config::RAM_SIZE],
            registers: [0; config::REGISTER_SIZE],
            address_register: 0,
            program_counter: 0,
            stack: [0; config::STACK_SIZE],
            stack_pointer: 0,
            timer_delay: 0,
            timer_sound: 0,
            keyboard: [false; config::KEYBOARD_SIZE],
        }
    }

    pub fn run(&mut self, rom_path: &String) {
        // Print an error if ROM loading failed
        if let Err(_) = self.load(rom_path) {
            println!("Failed to load ROM \"{}\"", &rom_path);
            return;
        }

        // Main run loop for the emulator
        loop {
            let opcode = {
                let msb = self.ram[self.program_counter] as u16;
                let lsb = self.ram[self.program_counter + 1] as u16;
                (msb << 8) | lsb
            };
            self.program_counter += 2;
            self.interpret(opcode);
            self.renderer.render(&self.display);

            // TODO: remove this 1fps limit
            thread::sleep(Duration::from_secs(1));
        }
    }

    fn load(&mut self, rom_path: &String) -> Result<(), Error> {
        File::open(rom_path)?.read(&mut self.ram)?;
        Ok(())
    }
}
