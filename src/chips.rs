const PC: usize = 0x200;

pub struct Chips {
    memory: [u8; 4096],
}

impl Default for Chips {
    fn default() -> Self {
        let mut memory = [0; 4096];

        memory[0] = 0b01100000;
        memory[1] = 0b10010000;
        memory[2] = 0b10010000;
        memory[3] = 0b01100000;

        memory[4] = 0b01100000;
        memory[5] = 0b10010000;
        memory[6] = 0b10010000;
        memory[7] = 0b01100000;

        memory[8] = 0b01100000;
        memory[9] = 0b10010000;
        memory[10] = 0b10010000;
        memory[11] = 0b01100000;
        Self { memory }
    }
}

impl Chips {
    pub fn load_bin(&mut self, bytes: &[u8]) {
        println!("Loading binary...");
        for (i, b) in bytes.iter().enumerate() {
            println!("opcode: {b:x}");
            self.memory[PC + i] = *b;
        }
    }

    pub fn memory_as_pixels(&self, width: usize, height: usize) -> Vec<u32> {
        let mut buf = vec![0; width * height];

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                if idx < self.memory.len() {
                    buf[idx] = if self.memory[idx] != 0 {
                        0xffffffff
                    } else {
                        0x00000000
                    };
                }
            }
        }
        buf
    }
}
