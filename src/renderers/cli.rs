use super::super::traits::Renderer;
use termion::clear;

pub struct CLI;

impl Renderer for CLI {
    fn render(&mut self, display: &[bool], width: &usize, height: &usize) {
        println!("{}", clear::All);
        println!("{}{}{}", "╔", format!("{:═<1$}", "", width), "╗");
        for y in 0..*height {
            let mut line: String = String::new();
            line += "║";
            for x in 0..*width {
                line += match display[width * y + x] {
                    true => "█",
                    false => " ",
                }
            }
            line += "║";
            println!("{}", line);
        }
        println!("{}{}{}", "╚", format!("{:═<1$}", "", width), "╝");
    }
}
