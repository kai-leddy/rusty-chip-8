use super::Renderable;
use super::super::chip8::display::Display;

pub struct CLI;

impl Renderable for CLI {
    fn render(&mut self, display: &Display) {
        // Clear the terminal
        print!("{}[2J", 27 as char);
        // Draw the top of the border
        println!(
            "{}{}{}",
            "╔",
            format!("{:═<1$}", "", display.width()),
            "╗"
        );
        // Draw each display cell
        for y in 0..display.height() {
            let mut line: String = String::new();
            line += "║";
            for x in 0..display.width() {
                line += match display.get(x, y) {
                    true => "█",
                    false => " ",
                }
            }
            line += "║";
            println!("{}", line);
        }
        // Draw the bottom border
        println!(
            "{}{}{}",
            "╚",
            format!("{:═<1$}", "", display.width()),
            "╝"
        );
    }
}
