pub mod chip8 {
    use std::fs::File;
    use std::io::Read;

    const STARTING_ADDRESS: usize = 0x200;
    const FONTSET_STARTING_ADDRESS: usize = 0x50;

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
        video: [u32; 64 * 32],
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

        pub fn op_00e0(mut self) -> () {
            self.video = [0; 64 * 32];
        }

        pub fn op_00ee(mut self) -> () {
            self.sp -= 1;
            self.pc = self.stack[self.sp as usize];
        }

        pub fn op_1nnn(mut self) -> () {
            let address: u16 = self.opcode & 0x0FFF;
            self.pc = address;
        }

        pub fn op_2nnn(mut self) -> () {
            let address: u16 = self.opcode & 0x0FFF;
            self.stack[self.sp as usize] = self.pc;
            self.sp += 1;
            self.pc = address;
        }

        pub fn op_3xkk(mut self) -> () {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let byte: u16 = self.opcode & 0x00FF;
            if self.registers[vx as usize] == byte as u8 {
                self.pc += 2;
            }
        }

        pub fn op_4xkk(mut self) -> () {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let byte: u16 = self.opcode & 0x00FF;
            if self.registers[vx as usize] != byte as u8 {
                self.pc += 2;
            }
        }

        pub fn op_5xy0(mut self) -> () {
            let vx: u16 = (self.opcode & 0x0F00) >> 8;
            let vy: u16 = (self.opcode & 0x00F0) >> 4;
            if self.registers[vx as usize] != self.registers[vy as usize] {
                self.pc += 2;
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
                video: [0; 64 * 32],
                opcode: 0,
            };

            chip8.memory[FONTSET_STARTING_ADDRESS..FONTSET_STARTING_ADDRESS + FONTSET.len()]
                .copy_from_slice(&FONTSET);

            chip8
        }
    }
}
