use super::Renderable;

pub struct CLI;

impl Renderable for CLI {
    fn render(&mut self, display: &[bool], width: &usize, height: &usize) {
        // Clear the terminal
        print!("{}[2J", 27 as char); 
        // Draw the top of the border
        println!("{}{}{}", "╔", format!("{:═<1$}", "", width), "╗");
        // Draw each display cell
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
        // Draw the bottom border
        println!("{}{}{}", "╚", format!("{:═<1$}", "", width), "╝");
    }
}
