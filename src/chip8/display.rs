use super::super::config;

pub struct Display {
    screen: [[bool; config::DISPLAY_HEIGHT]; config::DISPLAY_WIDTH],
}

impl Display {
    fn wrap_coords(x: usize, y: usize) -> (usize, usize) {
        let x = x % config::DISPLAY_WIDTH;
        let y = y % config::DISPLAY_HEIGHT;
        (x, y)
    }

    pub fn new() -> Display {
        Display {
            screen: [[false; config::DISPLAY_HEIGHT]; config::DISPLAY_WIDTH],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        let (x, y) = Display::wrap_coords(x, y);
        self.screen[x][y]
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        let (x, y) = Display::wrap_coords(x, y);
        self.screen[x][y] = value
    }

    pub fn width(&self) -> usize {
        config::DISPLAY_WIDTH
    }

    pub fn height(&self) -> usize {
        config::DISPLAY_HEIGHT
    }
}
