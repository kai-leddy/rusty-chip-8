mod config;
mod chip8;
mod renderers;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let mut renderer = renderers::cli::CLI;
            let mut emulator = chip8::Chip8::new(&mut renderer);
            emulator.run(&args[1]);
        }
        _ => println!("Usage: rusty-chip-8 <path_to_rom>"),
    }
}
