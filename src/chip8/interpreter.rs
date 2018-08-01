use super::Chip8;

impl<'a> Chip8<'a> {
    pub(super) fn interpret(&mut self, opcode: u16) {
        match opcode {
            // 00E0 -> Clear the display
            0x00e0 => {
                self.display.clear();
            }
            // 00EE -> Return from a subroutine
            0x00ee => {}
            // ANNN -> Set address register I to NNN
            0xa000...0xafff => {
                self.address_register = self.get_nnn(&opcode);
            }
            // DXYN -> Draw N height sprite at (X,Y)
            0xd000...0xdfff => {
                // Draw sprite
                let x = self.get_x(&opcode) as usize;
                let y = self.get_y(&opcode) as usize;
                let n = self.get_n(&opcode) as usize;

                let i = self.address_register;
                for dy in 0..n {
                    // get each byte from ram
                    let byte = self.ram[i + dy as usize];
                    // get each bit from the byte
                    for dx in 0..8 {
                        // get as a boolean
                        let sprite_cell = (byte & (1 << (7 - dx))) != 0;
                        // convenience variables
                        let x = x + dx;
                        let y = y + dy;
                        // current cell value
                        let current_cell = self.display.get(x, y);
                        // XOR with sprite cell value
                        self.display.set(x, y, current_cell ^ sprite_cell);
                        // set VF for flipped bits
                        self.registers[0xf] = (current_cell & sprite_cell) as u8;
                    }
                }
            }
            _ => (),
        }
    }

    fn get_x(&self, opcode: &u16) -> u8 {
        let x = (opcode & 0x0f00) >> 8;
        self.registers[x as usize]
    }

    fn get_y(&self, opcode: &u16) -> u8 {
        let y = (opcode & 0x00f0) >> 4;
        self.registers[y as usize]
    }

    fn get_n(&self, opcode: &u16) -> u8 {
        (opcode & 0x000f) as u8
    }

    fn get_nn(&self, opcode: &u16) -> u8 {
        (opcode & 0x00ff) as u8
    }

    fn get_nnn(&self, opcode: &u16) -> usize {
        (opcode & 0x0fff) as usize
    }
}
