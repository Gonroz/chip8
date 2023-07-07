use bevy::prelude::*;
use rand::Rng;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

mod test;
mod util;

const PIXEL_SIZE: f32 = 17.0;
const PIXEL_GAP: f32 = 1.0;

#[derive(Bundle)]
struct Pixel {
    sprite_bundle: SpriteBundle,
    position: Position,
}

#[derive(Component)]
struct Position {
    x: usize,
    y: usize,
}

impl Pixel {
    fn new(pos_x: f32, pos_y: f32, x: usize, y: usize) -> Self {
        // println!("New pixel");
        Pixel {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::BLACK,
                    custom_size: Some(Vec2::new(PIXEL_SIZE, PIXEL_SIZE)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 0.0)),
                ..default()
            },
            position: Position { x, y },
        }
    }
}

pub struct Chip8Plugin;

impl Plugin for Chip8Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins)
            // .add_startup_system(setup)
            .add_startup_system(setup)
            .add_systems((input, update.after(input), draw.after(update)));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(Chip8::new());
    for y in 0..32 {
        for x in 0..64 {
            // println!("spawn");
            commands.spawn(Pixel::new(
                x as f32 * (PIXEL_SIZE + PIXEL_GAP) - (PIXEL_SIZE + PIXEL_GAP) * 32.0,
                -(y as f32 * (PIXEL_SIZE + PIXEL_GAP)) + (PIXEL_SIZE + PIXEL_GAP) * 16.0,
                x,
                y,
            ));
        }
    }
}

fn draw(mut pixel_query: Query<(&mut Sprite, &mut Position)>, chip8_query: Query<&mut Chip8>) {
    // lala
    // println!("{}", chip8_query.is_empty());
    let chip8 = chip8_query.single();
    for mut pixel in pixel_query.iter_mut() {
        if chip8.screen[pixel.1.x][pixel.1.y] == 0 {
            pixel.0.color = Color::BLACK;
        } else {
            pixel.0.color = Color::WHITE;
        }
    }
    // for (mut sprite, position, chip8) in query.iter_mut() {
    //     if chip8.screen[position.x][position.y] == 0 {
    //         println!("Black");
    //         sprite.color = Color::BLACK;
    //     } else {
    //         println!("White");
    //         sprite.color = Color::WHITE;
    //     }
    // }
}

fn update(mut query: Query<&mut Chip8>) {
    // lala
    // println!("{}", query.is_empty());
    query.single_mut().tick();
}

fn input(mut chip8_query: Query<&mut Chip8>, keys: Res<Input<KeyCode>>) {
    for key in keys.get_pressed() {
        // lala
        chip8_query.single_mut().keyboard = util::keycode_to_hex(*key);
    }
}

// fn init_chip8(mut query: Query<&mut Chip8>) {
//     println!("{}", query.is_empty());
//     // query.single_mut().init();
// }

#[derive(Component)]
struct Chip8 {
    ram: [u8; 4096],
    registers: [u8; 16],
    i: u16,
    vf: u8,
    keyboard: u8,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: usize,
    stack: [u16; 16],
    screen: [[u8; 32]; 64],
}

impl Chip8 {
    fn new() -> Self {
        // return default
        Chip8 {
            ram: Chip8::ram_init(),
            registers: [0; 16],
            i: 0,
            vf: 0,
            delay_timer: 0,
            sound_timer: 0,
            keyboard: 0xFF,
            program_counter: 0x200,
            stack_pointer: 0,
            stack: [0; 16],
            screen: [[0; 32]; 64],
        }
    }

    fn ram_init() -> [u8; 4096] {
        let mut ram: [u8; 4096] = [0; 4096];
        // Load font into memory
        for i in 0..util::CHIP8_FONT.len() {
            ram[i] = util::CHIP8_FONT[i];
        }
        Chip8::load(&mut ram);
        return ram;
    }

    fn load(ram: &mut [u8; 4096]) {
        // load data in
        println!("{}", std::env::current_dir().unwrap().display());
        let rom_name = "test_opcode.ch8".to_string();
        let file = File::open(rom_name).expect("Couldn't find file.");
        let mut reader = BufReader::new(file);
        let mut buffer: Vec<u8> = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Failed to read file.");
        for i in 0..buffer.len() {
            ram[0x200 + i] = buffer[i];
        }

        // for i in 0..data.len() {
        //     // add data to ram
        //     self.ram[0x200 + i] = data[i];
        // }
    }

    fn tick(&mut self) {
        // println!("tick");
        // do stuff
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        let opcode: u16 = (self.ram[self.program_counter as usize] as u16) << 8
            | (self.ram[self.program_counter as usize + 1] as u16);
        self.perform_opcode(opcode);
    }

    // fn load_font_into_memory(&mut self) {
    //     // load font into memory
    //     for i in 0..util::CHIP8_FONT.len() {
    //         self.ram[i] = util::CHIP8_FONT[i];
    //     }
    // }

    fn perform_opcode(&mut self, opcode: u16) {
        let nibbles = (
            (opcode & 0xF000) >> 12 as usize,
            (opcode & 0x0F00) >> 8 as usize,
            (opcode & 0x00F0) >> 4 as usize,
            (opcode & 0x000F) as usize,
        );

        let nnn = opcode & 0x0FFF;
        let n: usize = nibbles.3;
        // let x: usize = opcode as usize & 0x0F00;
        let x: usize = nibbles.1 as usize;
        let y: usize = nibbles.2 as usize;
        let kk: u8 = (opcode & 0x00FF) as u8;

        // Perform specific opcode
        match nibbles {
            (0x0, 0x0, 0xE, 0x0) => self.opcode_00E0(),
            (0x0, 0x0, 0xE, 0xE) => self.opcode_00EE(),
            (0x1, _, _, _) => self.opcode_1nnn(nnn),
            (0x2, _, _, _) => self.opcode_2nnn(nnn),
            (0x3, _, _, _) => self.opcode_3xkk(x, kk),
            (0x4, _, _, _) => self.opcode_4xkk(x, kk),
            (0x5, _, _, 0x0) => self.opcode_5xy0(x, y),
            (0x6, _, _, _) => self.opcode_6xkk(x, kk),
            (0x7, _, _, _) => self.opcode_7xkk(x, kk),
            (0x8, _, _, 0x0) => self.opcode_8xy0(x, y),
            (0x8, _, _, 0x1) => self.opcode_8xy1(x, y),
            (0x8, _, _, 0x2) => self.opcode_8xy2(x, y),
            (0x8, _, _, 0x3) => self.opcode_8xy3(x, y),
            (0x8, _, _, 0x4) => self.opcode_8xy4(x, y),
            (0x8, _, _, 0x5) => self.opcode_8xy5(x, y),
            (0x8, _, _, 0x6) => self.opcode_8xy6(x),
            (0x8, _, _, 0x7) => self.opcode_8xy7(x, y),
            (0x8, _, _, 0xE) => self.opcode_8xyE(x, y),
            (0x9, _, _, 0x0) => self.opcode_9xy0(x, y),
            (0xA, _, _, _) => self.opcode_Annn(nnn),
            (0xB, _, _, _) => self.opcode_Bnnn(nnn),
            (0xC, _, _, _) => self.opcode_Cxkk(x, kk),
            (0xD, _, _, _) => self.opcode_Dxyn(x, y, n),
            (0xE, _, 0x9, 0xE) => self.opcode_Ex9E(x),
            (0xE, _, 0xA, 0x1) => self.opcode_ExA1(x),
            (0xF, _, 0x0, 0x7) => self.opcode_Fx07(x),
            (0xF, _, 0x0, 0xA) => self.opcode_Fx0A(x),
            (0xF, _, 0x1, 0x5) => self.opcode_Fx15(x),
            (0xF, _, 0x1, 0x8) => self.opcode_Fx18(x),
            (0xF, _, 0x1, 0xE) => self.opcode_Fx1E(x),
            (0xF, _, 0x2, 0x9) => self.opcode_Fx29(x),
            (0xF, _, 0x3, 0x3) => self.opcode_Fx33(x),
            (0xF, _, 0x5, 0x5) => self.opcode_Fx55(x),
            (0xF, _, 0x6, 0x5) => self.opcode_Fx65(x),
            _ => panic!("invalid opcode"),
        }
    }

    fn increment_program_counter(&mut self, num: u16) {
        self.program_counter = self.program_counter + num * 2;
    }

    // CLS
    fn opcode_00E0(&mut self) {
        // Clear the display.
        for row in 0..self.screen.len() {
            for column in 0..self.screen[0].len() {
                self.screen[row][column] = 0;
            }
        }
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
        } else {
            self.increment_program_counter(1);
        }
    }

    // SNE Vx, byte
    fn opcode_4xkk(&mut self, x: usize, kk: u8) {
        // Skip next instruction if Vx != kk.

        // The interpreter compares register Vx to kk, and if they are not equal,
        // increments the program counter by 2.

        if self.registers[x] != kk {
            self.increment_program_counter(2);
        } else {
            self.increment_program_counter(1);
        }
    }

    fn opcode_5xy0(&mut self, x: usize, y: usize) {
        //  Skip next instruction if Vx = Vy.

        // The interpreter compares register Vx to register Vy, and if they are equal,
        // increments the program counter by 2.

        if self.registers[x] == self.registers[y] {
            self.increment_program_counter(2);
        } else {
            self.increment_program_counter(1);
        }
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

        self.registers[x] = self.registers[x].wrapping_add(kk);
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
    fn opcode_Dxyn(&mut self, x: usize, y: usize, n: usize) {
        // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.

        // The interpreter reads n bytes from memory, starting at the address stored in I.
        // These bytes are then displayed as sprites on screen at coordinates (Vx, Vy).
        // Sprites are XORed onto the existing screen.
        // If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0.
        // If the sprite is positioned so part of it is outside the coordinates of the display,
        // it wraps around to the opposite side of the screen.
        // See instruction 8xy3 for more information on XOR, and section 2.4,
        // Display, for more information on the Chip-8 screen and sprites.

        self.vf = 0;
        let x: usize = self.registers[x] as usize;
        let y: usize = self.registers[y] as usize;

        // for byte in 0..n {
        //     // do stuff
        //     for bit_shift in 0..8 {
        //         // do something with the bits
        //         let bit = (self.ram[self.i as usize + byte] >> (7 - bit_shift)) & 0x1;
        //         let cx = (x + byte) % 64;
        //         let cy = (y + bit_shift) % 32;
        //         if self.screen[cy][cx] == 1 && bit == 1 {
        //             self.screen[cy][cx] = 0;
        //             self.vf = 1;
        //         } else {
        //             self.screen[cy][cx] = bit;
        //         }
        //     }
        // }

        for byte in 0..n {
            let y = (self.ram[y] as usize + byte) % 64;
            for bit in 0..8 {
                let x = (self.ram[x] as usize + bit) % 32;
                let color = (self.ram[self.i as usize + byte] >> (7 - bit)) & 1;
                self.vf |= color & self.screen[y][x];
                self.screen[y][x] ^= color;
            }
        }

        self.increment_program_counter(1);
    }

    // SKP Vx
    fn opcode_Ex9E(&mut self, x: usize) {
        // Skip next instruction if key with the value of Vx is pressed.

        // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position,
        // PC is increased by 2.

        let mut matching_key_pressed = false;
        if self.registers[x] == self.keyboard {
            matching_key_pressed = true;
        }

        if matching_key_pressed {
            self.increment_program_counter(2);
        } else {
            self.increment_program_counter(1);
        }
    }

    // SKNP A1
    fn opcode_ExA1(&mut self, x: usize) {
        // Skip next instruction if key with the value of Vx is not pressed.

        // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.

        let mut matching_key_pressed = false;
        if self.registers[x] == self.keyboard {
            matching_key_pressed = true;
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
    fn opcode_Fx0A(&mut self, x: usize) {
        // Wait for a key press, store the value of the key in Vx.

        // All execution stops until a key is pressed, then the value of that key is stored in Vx.

        if self.keyboard != 0xFF {
            self.increment_program_counter(1);
        }
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

        // The value of I is set to the location in memory for the hexadecimal sprite corresponding to the value of Vx.
        // See section 2.4, Display, for more information on the Chip-8 hexadecimal font.

        self.i = self.ram[x * 5] as u16;
        self.increment_program_counter(1);
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
