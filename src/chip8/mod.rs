use bevy::prelude::*;
use rand::Rng;

mod util;

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

        if self.registers[x] > 127 {
            self.vf = 1;
        } else {
            self.vf = 0;
        }
        self.registers[x] = self.registers[x] * 2;
        self.increment_program_counter(1);
    }

    // SNE Vx, Vy
    fn opcode_9xy0(&mut self, x: usize, y: usize) {
        // Skip next instruction if Vx != Vy.

        // The values of Vx and Vy are compared, and if they are not equal,
        // the program counter is increased by 2.

        if self.registers[x] != self.registers[y] {
            self.increment_program_counter(2);
        } else {
            self.increment_program_counter(1);
        }
    }

    // LD I, addr
    fn opcode_Annn(&mut self, nnn: u16) {
        // Set I = nnn.

        // The value of register I is set to nnn.

        self.i = nnn;
        self.increment_program_counter(1);
    }

    // JP V0, addr
    fn opcode_Bnnn(&mut self, nnn: u16) {
        // Jump to location nnn + V0.

        // The program counter is set to nnn plus the value of V0.

        self.program_counter = nnn + self.registers[0] as u16;
    }

    // RND Vx, byte
    fn opcode_Cxkk(&mut self, x: usize, kk: u8) {
        // Set Vx = random byte AND kk.

        // The interpreter generates a random number from 0 to 255,
        // which is then ANDed with the value kk. The results are stored in Vx.
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..256) as u8;
        self.registers[x] = num & kk;
        self.increment_program_counter(2);
    }

    // DRW Vx, Vy, nibble
    fn opcode_Dxyn(&mut self, x: usize, y: usize, n: u8) {
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

        // The interpreter reads n bytes from memory, starting at the address stored in I.
        // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
        // Sprites are XORed onto the existing screen.
        // If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
        // If the sprite is positioned so part of it is outside the coordinates of the display,
        // it wraps around to the opposite side of the screen.
        // See instruction 8xy3 for more information on XOR, and section 2.4,
        // Display, for more information on the Chip-8 screen and sprites.
    }

    // SKP Vx
    fn opcode_9x9E(&mut self, x: usize, keys: Res<Input<KeyCode>>) {
        // Skip next instruction if key with the value of Vx is pressed.

        // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position,
        // PC is increased by 2.

        let mut matching_key_pressed = false;
        for key in keys.get_pressed() {
            if self.registers[x] == util::keycode_to_hex(*key) {
                matching_key_pressed = true;
                break;
            }
        }

        if matching_key_pressed {
            self.increment_program_counter(2);
        } else {
            self.increment_program_counter(1);
        }
    }

    // SKNP A1
    fn opcode_9xA1(&mut self, x: usize, keys: Res<Input<KeyCode>>) {
        // Skip next instruction if key with the value of Vx is not pressed.

        // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.

        let mut matching_key_pressed = false;
        for key in keys.get_pressed() {
            if self.registers[x] == util::keycode_to_hex(*key) {
                matching_key_pressed = true;
                break;
            }
        }

        if matching_key_pressed {
            self.increment_program_counter(1);
        } else {
            self.increment_program_counter(2);
        }
    }

    // Lx Vx, DT
    fn opcode_Fx07(&mut self, x: usize) {
        // Set Vx = delay timer value.

        // The value of DT is placed into Vx.

        self.registers[x] = self.delay_timer;
        self.increment_program_counter(1);
    }

    // LD Vx, K
    fn opcode_Fx0A(&mut self, x: usize, keys: Res<Input<KeyCode>>) {
        // Wait for a key press, store the value of the key in Vx.

        // All execution stops until a key is pressed, then the value of that key is stored in Vx.

        let mut key_pressed = false;
        while !key_pressed {
            // do stuff
            for key in keys.get_pressed() {
                match key {
                    KeyCode::Key1 => {
                        self.registers[x] = util::keycode_to_hex(*key);
                        key_pressed = true;
                    }
                    KeyCode::Key2 => {
                        self.registers[x] = 0x2;
                        key_pressed = true;
                    }
                    KeyCode::Key3 => {
                        self.registers[x] = 0x3;
                        key_pressed = true;
                    }
                    KeyCode::Key4 => {
                        self.registers[x] = 0xC;
                        key_pressed = true;
                    }
                    KeyCode::Q => {
                        self.registers[x] = 0x4;
                        key_pressed = true;
                    }
                    KeyCode::W => {
                        self.registers[x] = 0x5;
                        key_pressed = true;
                    }
                    KeyCode::E => {
                        self.registers[x] = 0x6;
                        key_pressed = true;
                    }
                    KeyCode::R => {
                        self.registers[x] = 0xD;
                        key_pressed = true;
                    }
                    KeyCode::A => {
                        self.registers[x] = 0x7;
                        key_pressed = true;
                    }
                    KeyCode::S => {
                        self.registers[x] = 0x8;
                        key_pressed = true;
                    }
                    KeyCode::D => {
                        self.registers[x] = 0x9;
                        key_pressed = true;
                    }
                    KeyCode::F => {
                        self.registers[x] = 0xE;
                        key_pressed = true;
                    }
                    KeyCode::Z => {
                        self.registers[x] = 0xA;
                        key_pressed = true;
                    }
                    KeyCode::X => {
                        self.registers[x] = 0x0;
                        key_pressed = true;
                    }
                    KeyCode::C => {
                        self.registers[x] = 0xB;
                        key_pressed = true;
                    }
                    KeyCode::V => {
                        self.registers[x] = 0xF;
                        key_pressed = true;
                    }
                    _ => continue,
                }
            }
        }
        self.increment_program_counter(1);
    }

    // LD DT, Vx
    fn opcode_Fx15(&mut self, x: usize) {
        // Set delay timer = Vx.

        // DT is set equal to the value of Vx.

        self.delay_timer = self.registers[x];
        self.increment_program_counter(1);
    }

    // LD ST, Vx
    fn opcode_Fx18(&mut self, x: usize) {
        // Set sound timer = Vx.

        // ST is set equal to the value of Vx.

        self.sound_timer = self.registers[x];
        self.increment_program_counter(1);
    }

    // ADD I, Vx
    fn opcode_Fx1E(&mut self, x: usize) {
        // Set I = I + Vx.

        // The values of I and Vx are added, and the results are stored in I.

        self.i = self.i + self.registers[x] as u16;
        self.increment_program_counter(1);
    }

    // LD F, Vx
    fn opcode_Fx29(&mut self, x: usize) {
        // Set I = location of sprite for digit Vx.

        // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx.
        // See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
    }

    // LD B, Vx
    fn opcode_Fx33(&mut self, x: usize) {
        // Store BCD representation of Vx in memory locations I, I+1, and I+2.

        // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I,
        // the tens digit at location I+1, and the ones digit at location I+2.
        self.ram[self.i as usize] = self.registers[x] / 100;
        self.ram[self.i as usize + 1] = self.registers[x] % 100 / 10;
        self.ram[self.i as usize + 2] = self.registers[x] % 10;
    }

    // LD [I], Vx
    fn opcode_Fx55(&mut self, x: usize) {
        // Store registers V0 through Vx in memory starting at location I.

        // The interpreter copies the values of registers V0 through Vx into memory,
        // starting at the address in I.
        for i in 0..=x {
            self.ram[self.i as usize + i] = self.registers[i];
        }
        self.increment_program_counter(1);
    }

    // LD Vx, [I]
    fn opcode_Fx65(&mut self, x: usize) {
        // Read registers V0 through Vx from memory starting at location I.

        // The interpreter reads values from memory starting at location I into registers V0 through Vx.
        for i in 0..=x {
            self.registers[i] = self.ram[self.i as usize + i];
        }
        self.increment_program_counter(1);
    }
}
