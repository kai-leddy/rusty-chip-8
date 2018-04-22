use super::super::config;

pub struct Display {
    screen: [[bool; config::DISPLAY_HEIGHT]; config::DISPLAY_WIDTH],
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: [[false; config::DISPLAY_HEIGHT]; config::DISPLAY_WIDTH],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        self.screen[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        self.screen[x][y] = value
    }

    pub fn width(&self) -> usize {
        config::DISPLAY_WIDTH
    }

    pub fn height(&self) -> usize {
        config::DISPLAY_HEIGHT
    }
}
