use super::Chip8;

impl<'a> Chip8<'a> {
    pub(super) fn interpret(&mut self, opcode: u16) {
        match opcode {
            0xa000...0xafff => {
                self.address_register = self.get_nnn(&opcode);
            }
            0xd000...0xdfff => {
                // Draw sprite
                let x = self.get_x(&opcode) as usize;
                let y = self.get_y(&opcode) as usize;
                let n = self.get_n(&opcode) as usize;

                // TODO: get rid of the 0x200 offset hack
                let i = self.address_register - 0x200;
                for dy in 0..n {
                    // get each byte from ram
                    let byte = self.ram[i + dy as usize];
                    // get each bit from the byte
                    for dx in 0..8 {
                        // get as a boolean
                        let val = (byte & (1 << (7 - dx))) != 0;
                        self.display.set(x + dx, y + dy, val);
                        // TODO: set VF for flipped bits
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
