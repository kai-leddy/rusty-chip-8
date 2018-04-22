use super::Chip8;

impl<'a> Chip8<'a> {
    pub(super) fn interpret(&mut self, opcode: u16) {
        match opcode {
            0xD000...0xDFFF => {
                // let x = self.read_x(&opcode);
                // let y = self.read_y(&opcode);
                // let n = self.read_n(&opcode);
            }
            _ => (),
        }
    }
}
