pub mod chip8_emu {
    use std::fs::File;
    use std::io::Read;

    const STARTING_ADDRESS: usize = 0x200;
    const FONTSET_STARTING_ADDRESS: usize = 0x50;
    const VIDEO_WIDTH: usize = 64;
    const VIDEO_HEIGHT: usize = 32;

    const FONTSET: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

    pub struct Chip8 {
        registers: [u8; 16],
        memory: [u8; 4096],
        index: u16,
        pc: u16,
        stack: [u16; 16],
        sp: u8,
        delay_timer: u8,
        sound_timer: u8,
        keypad: [u8; 16],
        video: [u32; VIDEO_WIDTH * VIDEO_HEIGHT],
        opcode: u16,
    }

    impl Chip8 {
        pub fn load_rom(mut self, filename: &str) -> std::io::Result<()> {
            let mut file: File = File::open(filename)?;
            let mem: &mut [u8] = &mut self.memory[0x200..];
            let bytes_read = file.read(mem)?;
            println!("Read {} bytes", bytes_read);
            Ok(())
        }

        pub fn op_00e0(mut self) {
            self.video = [0; VIDEO_WIDTH * VIDEO_HEIGHT];
        }

        pub fn op_00ee(mut self) {
            self.sp -= 1;
            self.pc = self.stack[self.sp as usize];
        }

        pub fn op_1nnn(mut self) {
            let address: u16 = self.opcode & 0x0FFF;
            self.pc = address;
        }

        pub fn op_2nnn(mut self) {
            let address: u16 = self.opcode & 0x0FFF;
            self.stack[self.sp as usize] = self.pc;
            self.sp += 1;
            self.pc = address;
        }

        pub fn op_3xkk(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let byte: u16 = self.opcode & 0x00FF;
            if self.registers[vx as usize] == byte as u8 {
                self.pc += 2;
            }
        }

        pub fn op_4xkk(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let byte: u16 = self.opcode & 0x00FF;
            if self.registers[vx as usize] != byte as u8 {
                self.pc += 2;
            }
        }

        pub fn op_5xy0(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let vy: u16 = (self.opcode & 0x00F0) >> 4;
            if self.registers[vx as usize] != self.registers[vy as usize] {
                self.pc += 2;
            }
        }

        pub fn op_6xkk(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let byte: u16 = self.opcode & 0x00FF;
            self.registers[vx as usize] = byte as u8;
        }

        pub fn op_7xkk(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let byte: u16 = self.opcode & 0x00FF;
            self.registers[vx as usize] += byte as u8;
        }

        pub fn op_8xy0(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let vy: u16 = (self.opcode & 0x00F0) >> 4;
            self.registers[vx as usize] = self.registers[vy as usize];
        }

        pub fn op_8xy1(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let vy: u16 = (self.opcode & 0x00F0) >> 4;
            self.registers[vx as usize] |= self.registers[vy as usize];
        }

        pub fn op_8xy2(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let vy: u16 = (self.opcode & 0x00F0) >> 4;
            self.registers[vx as usize] &= self.registers[vy as usize];
        }

        pub fn op_8xy3(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let vy: u16 = (self.opcode & 0x00F0) >> 4;
            self.registers[vx as usize] ^= self.registers[vy as usize];
        }

        pub fn op_8xy4(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let vy: u16 = (self.opcode & 0x00F0) >> 4;
            let sum: u16 = self.registers[vx as usize] as u16 + self.registers[vy as usize] as u16;
            if sum > 255 {
                self.registers[0xF] = 1;
            } else {
                self.registers[0xF] = 0;
            }

            self.registers[vx as usize] = (sum & 0xFF) as u8;
        }

        pub fn op_8xy5(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let vy: u16 = (self.opcode & 0x00F0) >> 4;
            if self.registers[vx as usize] > self.registers[vy as usize] {
                self.registers[0xF] = 1;
            } else {
                self.registers[0xF] = 0;
            }

            self.registers[vx as usize] -= self.registers[vy as usize];
        }

        pub fn op_8xy6(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            self.registers[0xF] = self.registers[vx as usize] & 0x1;
            self.registers[vx as usize] >>= 1;
        }

        pub fn op_8xy7(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let vy: u16 = (self.opcode & 0x00F0) >> 4;
            if self.registers[vy as usize] > self.registers[vx as usize] {
                self.registers[0xF] = 1;
            } else {
                self.registers[0xF] = 0;
            }

            self.registers[vx as usize] = self.registers[vy as usize] - self.registers[vx as usize];
        }

        pub fn op_8xye(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            self.registers[0xF] = (self.registers[vx as usize] & 8) >> 7;
            self.registers[vx as usize] <<= 1;
        }

        pub fn op_9xy0(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let vy: u16 = (self.opcode & 0x00F0) >> 4;
            if self.registers[vx as usize] != self.registers[vy as usize] {
                self.pc += 2;
            }
        }

        pub fn op_annn(mut self) {
            let address: u16 = self.opcode & 0x0FFF;
            self.index = address;
        }

        pub fn op_bnnn(mut self) {
            let address: u16 = self.opcode & 0x0FFF;
            self.index = address;
            self.pc = self.registers[0] as u16 + address;
        }

        pub fn op_cxkk(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let byte: u16 = self.opcode & 0x00FF;
            let x: u8 = rand::random();
            self.registers[vx as usize] = x & byte as u8;
        }

        pub fn op_dxyn(mut self) {
            let vx: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
            let vy: u8 = ((self.opcode & 0x00F0) >> 4) as u8;
            let height: u8 = (self.opcode & 0x000F) as u8;
            let posx: u8 = self.registers[vx as usize] % VIDEO_WIDTH as u8;
            let posy: u8 = self.registers[vy as usize] % VIDEO_HEIGHT as u8;
            self.registers[0xF] = 0;
            for row in 0..height {
                let sprite_byte: u8 = self.memory[(self.index + row as u16) as usize];
                for col in 0..8 {
                    let sprite_pixel: u8 = sprite_byte & (0x80 >> col);
                    let screen_pixel: &mut u32 = &mut self.video[(posy as usize + row as usize) * VIDEO_WIDTH + (posx as usize + col)];

                    if sprite_pixel != 0x00000000 {
                        if *screen_pixel == 0xFFFFFFFF {
                            self.registers[0xF] = 1;
                        }
                        *screen_pixel ^= 0xFFFFFFFF;
                    }
                }
            }
        }

        pub fn op_ex9e(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let key: u8 = self.registers[vx as usize];
            if self.keypad[key as usize] != 0 {
                self.pc += 2;
            }
        }

        pub fn op_exa1(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let key: u8 = self.registers[vx as usize];
            if self.keypad[key as usize] == 0 {
                self.pc += 2;
            }
        }

        pub fn op_fx07(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            self.registers[vx as usize] = self.delay_timer;
        }

        pub fn op_fx0a(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;

            if self.keypad[0] != 0 {
                self.registers[vx as usize] = 0;
            }
            else if self.keypad[1] != 0 {
                self.registers[vx as usize] = 1;
            }
            else if self.keypad[2] != 0 {
                self.registers[vx as usize] = 2;
            }
            else if self.keypad[3] != 0 {
                self.registers[vx as usize] = 3;
            }
            else if self.keypad[4] != 0 {
                self.registers[vx as usize] = 4;
            }
            else if self.keypad[5] != 0 {
                self.registers[vx as usize] = 5;
            }
            else if self.keypad[6] != 0 {
                self.registers[vx as usize] = 6;
            }
            else if self.keypad[7] != 0 {
                self.registers[vx as usize] = 7;
            }
            else if self.keypad[8] != 0 {
                self.registers[vx as usize] = 8;
            }
            else if self.keypad[9] != 0 {
                self.registers[vx as usize] = 9;
            }
            else if self.keypad[10] != 0 {
                self.registers[vx as usize] = 10;
            }
            else if self.keypad[11] != 0 {
                self.registers[vx as usize] = 11;
            }
            else if self.keypad[12] != 0 {
                self.registers[vx as usize] = 12;
            }
            else if self.keypad[13] != 0 {
                self.registers[vx as usize] = 13;
            }
            else if self.keypad[14] != 0 {
                self.registers[vx as usize] = 14;
            }
            else if self.keypad[15] != 0 {
                self.registers[vx as usize] = 15;
            }
            else {
                self.pc -= 2;
            }
        } 

        pub fn op_fx15(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;

            self.delay_timer = self.registers[vx as usize];
        }
 
        pub fn op_fx18(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;

            self.sound_timer = self.registers[vx as usize];
        }
 
        pub fn op_fx1e(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;

            self.index = self.registers[vx as usize] as u16;
        }

        pub fn op_fx29(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let digit: u16 = self.registers[vx as usize] as u16;
            self.index = (FONTSET_STARTING_ADDRESS + (5 * digit) as usize) as u16;
        }

        pub fn op_fx33(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let mut value: u16 = self.registers[vx as usize] as u16;
 
            self.memory[(self.index + 2) as usize] = (value % 10) as u8;
            value /= 10;

            self.memory[(self.index + 1) as usize] = (value % 10) as u8;
            value /= 10;

            self.memory[self.index as usize] = (value % 10) as u8;
        }

        pub fn op_fx55(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;

            for i in 0..(vx + 1) {
                 self.memory[(self.index + i) as usize] = self.registers[i as usize];
            }
        }

        pub fn op_fx65(mut self) {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;

            for i in 0..(vx + 1) {
                self.registers[i as usize] = self.memory[(self.index + i) as usize];
            }
        }

        pub fn new() -> Chip8 {
            let mut chip8: Chip8 = Chip8 {
                registers: [0; 16],
                memory: [0; 4096],
                index: 0,
                pc: STARTING_ADDRESS as u16,
                stack: [0; 16],
                sp: 0,
                delay_timer: 0,
                sound_timer: 0,
                keypad: [0; 16],
                video: [0; VIDEO_WIDTH * VIDEO_HEIGHT],
                opcode: 0,
            };

            chip8.memory[FONTSET_STARTING_ADDRESS..FONTSET_STARTING_ADDRESS + FONTSET.len()]
                .copy_from_slice(&FONTSET);

            chip8
        }
    }
}
