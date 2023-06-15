use bevy::prelude::*;

pub struct Chip8Plugin;

impl Plugin for Chip8Plugin {
    fn build(&self, _app: &mut App) {
        // add things here
        let chip8 = Chip8::new();
    }
}

struct Chip8 {
    ram: [u8; 4096],
    registers: [u8; 16],
    i: u16,
    vf: u8,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: usize,
    stack: [u16; 16],
}

impl Chip8 {
    fn new() -> Self {
        // return default
        Chip8 {
            ram: [0; 4096],
            registers: [0; 16],
            i: 0,
            vf: 0,
            delay_timer: 0,
            sound_timer: 0,
            program_counter: 0,
            stack_pointer: 0,
            stack: [0; 16],
        }
    }

    fn load(&mut self, data: &Vec<u8>) {
        // load data in
        for i in 0..data.len() {
            // add data to ram
            self.ram[0x200 + i] = data[i];
        }
    }

    fn perform_opcode(&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xF000) >> 12 as usize,
            (opcode & 0x0F00) >> 8 as usize,
            (opcode & 0x00F0) >> 4 as usize,
            (opcode & 0x000F) as usize,
        );

        let nnn = opcode & 0x0FFF;
        let n = opcode & 0x000F;
        let x = opcode & 0x0F00;
        let y = opcode & 0x00F0;
        let kk = opcode & 0x00FF;

        // Perform specific opcode
        match nibbles {
            //lala
            //lala
            _ => panic!("invalid opcode"),
        }
    }

    fn increment_program_counter(&mut self, num: u16) {
        self.program_counter = self.program_counter + num;
    }

    // CLS
    fn opcode_00E0(&mut self) {
        // Clear the display.
        self.increment_program_counter(1);
    }

    // RET
    fn opcode_00EE(&mut self) {
        // Return from a subroutine.

        // The interpreter sets the program counter to the address at the top of the stack,
        // then subtracts 1 from the stack pointer.

        self.program_counter = self.stack[self.stack_pointer];
        self.stack_pointer = self.stack_pointer - 1;
    }

    // JP addr
    fn opcode_1nnn(&mut self, nnn: u16) {
        // Jump to location nnn.

        // The interpreter sets the program counter to nnn.

        self.program_counter = nnn;
    }

    // CALL addr
    fn opcode_2nnn(&mut self, nnn: u16) {
        // Call subroutine at nnn.

        // The interpreter increments the stack pointer, then puts the current PC on the top of the stack.
        // The PC is then set to nnn.

        self.stack_pointer = self.stack_pointer + 1;
        self.stack[self.stack_pointer] = self.program_counter;
        self.program_counter = nnn;
    }

    // SE Vx, byte
    fn opcode_3xkk(&mut self, x: usize, kk: u8) {
        // Skip next instruction if Vx = kk.

        // The interpreter compares register Vx to kk, and if they are equal,
        // increments the program counter by 2.

        if self.registers[x] == kk {
            self.increment_program_counter(2);
        }
        self.increment_program_counter(1);
    }

    // SNE Vx, byte
    fn opcode_4xkk(&mut self, x: usize, kk: u8) {
        // Skip next instruction if Vx != kk.

        // The interpreter compares register Vx to kk, and if they are not equal,
        // increments the program counter by 2.

        if self.registers[x] != kk {
            self.increment_program_counter(2);
        }
        self.increment_program_counter(1);
    }

    fn opcode_5xy0(&mut self, x: usize, y: usize) {
        //  Skip next instruction if Vx = Vy.

        // The interpreter compares register Vx to register Vy, and if they are equal,
        // increments the program counter by 2.

        if self.registers[x] == self.registers[y] {
            self.increment_program_counter(2);
        }
        self.increment_program_counter(1);
    }

    // LD Vx, byte
    fn opcode_6xkk(&mut self, x: usize, kk: u8) {
        // Set Vx = kk.

        // The interpreter puts the value kk into register Vx.

        self.registers[x] = kk;
        self.increment_program_counter(1);
    }

    // ADD Vx, byte
    fn opcode_7xkk(&mut self, x: usize, kk: u8) {
        // Set Vx = Vx + kk.

        // Adds the value kk to the value of register Vx, then stores the result in Vx.

        self.registers[x] = self.registers[x] + kk;
        self.increment_program_counter(1);
    }

    // LD Vx, Vy
    fn opcode_8xy0(&mut self, x: usize, y: usize) {
        // Set Vx = Vy.

        // Stores the value of register Vy in register Vx.

        self.registers[x] = self.registers[y];
        self.increment_program_counter(1);
    }

    // OR Vx, Vy
    fn opcode_8xy1(&mut self, x: usize, y: usize) {
        // Set Vx = Vx OR Vy.

        // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx.
        // A bitwise OR compares the corrseponding bits from two values, and if either bit is 1,
        // then the same bit in the result is also 1.
        // Otherwise, it is 0.

        self.registers[x] = self.registers[x | y];
        self.increment_program_counter(1);
    }

    // AND Vx, Vy
    fn opcode_8xy2(&mut self, x: usize, y: usize) {
        // Set Vx = Vx AND Vy.

        // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
        // A bitwise AND compares the corrseponding bits from two values, and if both bits are 1,
        // then the same bit in the result is also 1.
        // Otherwise, it is 0.

        self.registers[x] = self.registers[x & y];
        self.increment_program_counter(1);
    }

    // XOR Vx, Vy
    fn opcode_8xy3(&mut self, x: usize, y: usize) {
        // Set Vx = Vx XOR Vy.

        // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
        // An exclusive OR compares the corrseponding bits from two values,
        // and if the bits are not both the same, then the corresponding bit in the result is set to 1.
        // Otherwise, it is 0.

        self.registers[x] = self.registers[x ^ y];
        self.increment_program_counter(1);
    }

    // ADD Vx, Vy
    fn opcode_8xy4(&mut self, x: usize, y: usize) {
        // Set Vx = Vx + Vy, set VF = carry.

        // The values of Vx and Vy are added together.
        // If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
        // Only the lowest 8 bits of the result are kept, and stored in Vx.

        if self.registers[x] as u16 + self.registers[y] as u16 > 255 {
            self.vf = 1;
        }
        self.registers[x] = self.registers[x] + self.registers[y];
        self.increment_program_counter(1);
    }

    // SUB Vx, Vy
    fn opcode_8xy5(&mut self, x: usize, y: usize) {
        // Set Vx = Vx - Vy, set VF = NOT borrow.

        // If Vx > Vy, then VF is set to 1, otherwise 0.
        // Then Vy is subtracted from Vx, and the results stored in Vx.

        if self.registers[x] > self.registers[y] {
            self.vf = 1;
        } else {
            self.vf = 0;
        }
        self.registers[x] = self.registers[x] - self.registers[y];
        self.increment_program_counter(1);
    }

    // SHR Vx {, Vy}
    fn opcode_8xy6(&mut self, x: usize) {
        // Set Vx = Vx SHR 1.

        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
        // Then Vx is divided by 2.

        if self.registers[x] % 2 != 0 {
            self.vf = 1;
        } else {
            self.vf = 0;
        }
        self.registers[x] = self.registers[x] / 2;
        self.increment_program_counter(1);
    }

    // SUBN  Vx, Vy
    fn opcode_8xy7(&mut self, x: usize, y: usize) {
        // Set Vx = Vy - Vx, set VF = NOT borrow.

        // If Vy > Vx, then VF is set to 1, otherwise 0.
        // Then Vx is subtracted from Vy, and the results stored in Vx.

        if self.registers[y] > self.registers[x] {
            self.vf = 1;
        }
        self.registers[y] = self.registers[y] - self.registers[x];
        self.increment_program_counter(1);
    }

    // SHL Vx {, Vy}
    fn opcode_8xyE(&mut self, x: usize, y: usize) {
        // Set Vx = Vx SHL 1.

        // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
        // Then Vx is multiplied by 2.
    }
}
