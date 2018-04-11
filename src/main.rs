extern crate termion;

mod chip8;

use std::env;

fn main() {
    let mut emulator = chip8::Chip8::new();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => emulator.run(&args[1]),
        _ => println!("Usage: rusty-chip-8 <path_to_rom>"),
    }
}
