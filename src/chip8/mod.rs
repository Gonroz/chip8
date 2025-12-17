use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use std::time::Duration;

use rand::Rng;

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

// use std::{thread, time};

mod config;
use config::Config;

mod test;
mod theme;
use theme::Theme;

mod util;

mod ui;

const CHIP8_WIDTH: u32 = 64;
const CHIP8_HEIGHT: u32 = 32;
const SCREEN_SCALE_FACTOR: f32 = 16.0;

#[derive(Resource)]
struct ScreenHandle(Handle<Image>);

#[derive(Resource)]
struct ThemeColors {
    foreground_color: Color,
    background_color: Color,
}

#[derive(Resource)]
pub struct ResetFlag {
    reset: bool,
}

pub struct Chip8Plugin;

impl Plugin for Chip8Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
            .add_plugins(ui::UiPlugin)
            .add_message::<PlayPitch>()
            .add_systems(Startup, setup)
            .add_systems(Update, (input, reset, draw).chain())
            .add_systems(FixedUpdate, (update, sound).chain())
            .insert_resource(Time::<Fixed>::from_hz(60.0));
    }
}

fn setup(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut window: Single<&mut Window>,
) {
    window.resolution.set(
        CHIP8_WIDTH as f32 * SCREEN_SCALE_FACTOR + 100.0,
        CHIP8_HEIGHT as f32 * SCREEN_SCALE_FACTOR,
    );

    // First create a Config, then get the Theme
    let config: Config = Config::new();
    let theme: Theme = Theme::new(&config.theme);
    let rom_to_load: String = config.get_rom_path();
    // let chip8: Chip8 = Chip8::new_from_rom();
    commands.insert_resource(config);

    let theme_colors: ThemeColors = ThemeColors {
        foreground_color: Color::srgb_u8(
            theme.foreground[0],
            theme.foreground[1],
            theme.foreground[2],
        ),
        background_color: Color::srgb_u8(
            theme.background[0],
            theme.background[1],
            theme.background[2],
        ),
    };
    commands.insert_resource(theme);
    commands.insert_resource(theme_colors);

    let reset_flag: ResetFlag = ResetFlag { reset: false };
    commands.insert_resource(reset_flag);

    commands.spawn(Camera2d);

    // commands.insert_resource(Chip8::new());
    commands.insert_resource(Chip8::new(&rom_to_load));

    // Emulated Screen Stuff
    // we multiply by 4 because each pixel needs 4 bytes of data: 1 for red, green, blue, and alpha
    let data_len = ((CHIP8_WIDTH * CHIP8_HEIGHT) * 4) as usize;
    let initial_data: Vec<u8> = vec![0; data_len];

    let size = Extent3d {
        width: CHIP8_WIDTH,
        height: CHIP8_HEIGHT,
        depth_or_array_layers: 1,
    };

    let image = Image::new(
        size,
        TextureDimension::D2,
        initial_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::all(),
    );

    let image_handle = images.add(image);
    commands.insert_resource(ScreenHandle(image_handle.clone()));

    let mut screen = Sprite::from_image(image_handle);
    screen.custom_size = Some(Vec2::new(
        CHIP8_WIDTH as f32 * SCREEN_SCALE_FACTOR,
        CHIP8_HEIGHT as f32 * SCREEN_SCALE_FACTOR,
    ));

    commands.spawn(screen);
}

fn draw(
    mut chip8: ResMut<Chip8>,
    handle: Res<ScreenHandle>,
    mut images: ResMut<Assets<Image>>,
    theme_colors: Res<ThemeColors>,
) {
    if !chip8.screen_dirty {
        return;
    }

    let image = match images.get_mut(&handle.0) {
        Some(img) => img,
        None => return,
    };

    for y in 0..CHIP8_HEIGHT {
        for x in 0..CHIP8_WIDTH {
            let color_to_set = match chip8.screen[y as usize][x as usize] {
                1 => theme_colors.foreground_color,
                _ => theme_colors.background_color,
            };
            image
                .set_color_at(x, y, color_to_set)
                .expect("Could not set color.");
        }
    }

    chip8.screen_dirty = false;
}

fn update(mut chip8: ResMut<Chip8>, config: Res<Config>) {
    if chip8.delay_timer > 0 {
        chip8.delay_timer = chip8.delay_timer - 1;
    }
    if chip8.sound_timer > 0 {
        chip8.sound_timer = chip8.sound_timer - 1;
    }

    for _ in 0..config.instructions_per_second {
        chip8.tick();
    }

    // make sure to reset input so it can be gotten again
    chip8.keyboard.clear();
}

fn input(
    mut chip8: ResMut<Chip8>,
    input: Res<ButtonInput<KeyCode>>,
    mut reset_flag: ResMut<ResetFlag>,
) {
    if input.all_just_released([KeyCode::ControlLeft, KeyCode::KeyR]) {
        reset_flag.reset = true;
    }

    for key in input.get_pressed() {
        chip8.keyboard.push(util::keycode_to_hex(&key));
    }
}

#[derive(Message, Default)]
struct PlayPitch;

fn sound(
    mut commands: Commands,
    chip8: Res<Chip8>,
    config: Res<Config>,
    mut pitch_assets: ResMut<Assets<Pitch>>,
    // mut play_pitch_reader: MessageReader<PlayPitch>,
) {
    //blah
    if chip8.sound_timer > 0 {
        commands.spawn((
            AudioPlayer(pitch_assets.add(Pitch::new(
                config.pitch_frequency,
                Duration::from_millis(config.pitch_duration),
            ))),
            PlaybackSettings::DESPAWN,
        ));
    }
}

fn reset(
    mut chip8: ResMut<Chip8>,
    mut config: ResMut<Config>,
    mut theme: ResMut<Theme>,
    mut theme_colors: ResMut<ThemeColors>,
    mut reset_flag: ResMut<ResetFlag>,
) {
    if reset_flag.reset == false {
        return;
    }

    let temp_config: Config = Config::new();
    // let rom: String = temp_config.rom.clone();
    let temp_theme: Theme = Theme::new(&temp_config.theme);
    *config = temp_config;
    *theme = temp_theme;
    let temp_theme_colors: ThemeColors = ThemeColors {
        foreground_color: Color::srgb_u8(
            theme.foreground[0],
            theme.foreground[1],
            theme.foreground[2],
        ),
        background_color: Color::srgb_u8(
            theme.background[0],
            theme.background[1],
            theme.background[2],
        ),
    };
    *theme_colors = temp_theme_colors;

    *chip8 = Chip8::new(&config.get_rom_path());
    reset_flag.reset = false;
    // blah
}

#[derive(Component, Resource)]
struct Chip8 {
    ram: [u8; 4096],
    registers: [u8; 16],
    i: u16,
    keyboard: Vec<u8>,
    delay_timer: u8,
    sound_timer: u8,
    program_counter: u16,
    stack_pointer: usize,
    stack: [u16; 16],
    screen: [[u8; 64]; 32],
    screen_dirty: bool,
    shift_quirk_vx_eq_vy: bool,
}

impl Chip8 {
    fn new_no_rom() -> Self {
        // return default
        Chip8 {
            ram: Chip8::ram_init(),
            registers: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            keyboard: vec![],
            program_counter: 0x200,
            stack_pointer: 0,
            stack: [0; 16],
            screen: [[0; 64]; 32],
            screen_dirty: false,
            shift_quirk_vx_eq_vy: true,
        }
    }

    fn new(rom: &str) -> Self {
        Chip8 {
            ram: Chip8::ram_init_from_rom(rom),
            registers: [0; 16],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            keyboard: vec![],
            program_counter: 0x200,
            stack_pointer: 0,
            stack: [0; 16],
            screen: [[0; 64]; 32],
            screen_dirty: false,
            shift_quirk_vx_eq_vy: true,
        }
    }

    fn ram_init() -> [u8; 4096] {
        let mut ram: [u8; 4096] = [0; 4096];
        // Load font into memory
        for i in 0..util::CHIP8_FONT.len() {
            ram[i] = util::CHIP8_FONT[i];
        }
        // Chip8::load(&mut ram);
        return ram;
    }

    fn ram_init_from_rom(rom_path: &str) -> [u8; 4096] {
        let mut ram: [u8; 4096] = [0; 4096];
        // Load font into memory
        for i in 0..util::CHIP8_FONT.len() {
            ram[i] = util::CHIP8_FONT[i];
        }
        Chip8::load_from_path(&mut ram, rom_path);
        return ram;
    }

    // fn load(ram: &mut [u8; 4096]) {
    //     // load data in
    //     println!("{}", std::env::current_dir().unwrap().display());
    //     // let rom_name = util::get_rom_to_load();
    //     // let file = File::open(rom_name).expect("Couldn't find file. (mod.rs)");
    //     // let rom_path = util::get_rom_to_load();
    //     let file = File::open(&rom_path).expect(&format!(
    //         "Couldn't find ROM file at absolute path: {}",
    //         rom_path
    //     ));
    //     let mut reader = BufReader::new(file);
    //     let mut buffer: Vec<u8> = Vec::new();
    //     reader
    //         .read_to_end(&mut buffer)
    //         .expect("Failed to read file.");
    //     for i in 0..buffer.len() {
    //         ram[0x200 + i] = buffer[i];
    //     }
    // }

    fn load_from_path(ram: &mut [u8; 4096], rom_path: &str) {
        let file =
            File::open(rom_path).expect(&format!("Couldn't find ROM file at path: {}", rom_path));
        let mut reader = BufReader::new(file);
        let mut buffer: Vec<u8> = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .expect("Failed to read file.");
        for i in 0..buffer.len() {
            ram[0x200 + i] = buffer[i];
        }
    }

    fn tick(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        let opcode: u16 = (self.ram[self.program_counter as usize] as u16) << 8
            | (self.ram[self.program_counter as usize + 1] as u16);
        self.perform_opcode(opcode);

        // thread::sleep(time::Duration::from_millis(5));
    }

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
            (0x8, _, _, 0x6) => self.opcode_8xy6(x, y),
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
    #[allow(non_snake_case)]
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
    #[allow(non_snake_case)]
    fn opcode_00EE(&mut self) {
        // Return from a subroutine.

        // The interpreter sets the program counter to the address at the top of the stack,
        // then subtracts 1 from the stack pointer.

        self.program_counter = self.stack[self.stack_pointer];
        self.stack_pointer = self.stack_pointer - 1;
        self.increment_program_counter(1);

        // gemini suggestion code below
        // self.stack_pointer -= 1;
        // self.program_counter = self.stack[self.stack_pointer];
        // self.increment_program_counter(1);
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

        self.registers[x] |= self.registers[y];
        self.increment_program_counter(1);
    }

    // AND Vx, Vy
    fn opcode_8xy2(&mut self, x: usize, y: usize) {
        // Set Vx = Vx AND Vy.

        // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx.
        // A bitwise AND compares the corrseponding bits from two values, and if both bits are 1,
        // then the same bit in the result is also 1.
        // Otherwise, it is 0.

        self.registers[x] &= self.registers[y];
        self.increment_program_counter(1);
    }

    // XOR Vx, Vy
    fn opcode_8xy3(&mut self, x: usize, y: usize) {
        // Set Vx = Vx XOR Vy.

        // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx.
        // An exclusive OR compares the corrseponding bits from two values,
        // and if the bits are not both the same, then the corresponding bit in the result is set to 1.
        // Otherwise, it is 0.

        self.registers[x] ^= self.registers[y];
        self.increment_program_counter(1);
    }

    // ADD Vx, Vy
    fn opcode_8xy4(&mut self, x: usize, y: usize) {
        // Set Vx = Vx + Vy, set VF = carry.

        // The values of Vx and Vy are added together.
        // If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
        // Only the lowest 8 bits of the result are kept, and stored in Vx.

        // original code
        // self.registers[0xF] = 0;
        // if self.registers[x] as u16 + self.registers[y] as u16 > 255 {
        //     self.registers[0xF] = 1;
        // }
        // self.registers[x] = self.registers[x].wrapping_add(self.registers[y]);
        // self.increment_program_counter(1);

        // let sum: u16 = (self.registers[x] + self.registers[y]) as u16;
        // self.registers[0xF] = if sum > 0xFF { 1 } else { 0 };
        // self.registers[x] = (sum & 0xFF) as u8;
        let sum = self.registers[x].wrapping_add(self.registers[y]);
        self.registers[0xF] = if self.registers[x] as u16 + self.registers[y] as u16 > 255 {
            1
        } else {
            0
        };
        if x != 0xF {
            self.registers[x] = sum;
        };
        self.increment_program_counter(1);
    }

    // SUB Vx, Vy
    fn opcode_8xy5(&mut self, x: usize, y: usize) {
        // Set Vx = Vx - Vy, set VF = NOT borrow.

        // If Vx > Vy, then VF is set to 1, otherwise 0.
        // Then Vy is subtracted from Vx, and the results stored in Vx.

        // self.registers[0xF] = 0;
        // if self.registers[x] > self.registers[y] {
        //     self.registers[0xF] = 1;
        // }
        // self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]);
        // self.increment_program_counter(1);

        let difference = self.registers[x].wrapping_sub(self.registers[y]);
        self.registers[0xF] = if self.registers[x] >= self.registers[y] {
            1
        } else {
            0
        };
        if x != 0xF {
            self.registers[x] = difference;
        };
        self.increment_program_counter(1);
    }

    // SHR Vx {, Vy}
    fn opcode_8xy6(&mut self, x: usize, y: usize) {
        // Set Vx = Vx SHR 1.

        // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0.
        // Then Vx is divided by 2.
        let original_vx = self.registers[x];

        if self.shift_quirk_vx_eq_vy {
            self.registers[x] = self.registers[y];
        }

        // gemini code
        // let vx_value_before_shift = self.registers[x];

        // self.registers[0xF] = vx_value_before_shift & 0x1;
        // self.registers[x] >>= 1;
        // self.increment_program_counter(1);

        self.registers[0xF] = original_vx & 0x1;
        if x != 0xF {
            self.registers[x] >>= 1;
        }
        self.increment_program_counter(1);

        // my original code
        // self.registers[0xF] = 0;
        // if self.registers[x] % 2 != 0 {
        //     self.registers[0xF] = 1;
        // }
        // self.registers[x] = self.registers[x] / 2;
        // self.increment_program_counter(1);
    }

    // SUBN  Vx, Vy
    fn opcode_8xy7(&mut self, x: usize, y: usize) {
        // Set Vx = Vy - Vx, set VF = NOT borrow.

        // If Vy > Vx, then VF is set to 1, otherwise 0.
        // Then Vx is subtracted from Vy, and the results stored in Vx.

        // self.registers[0xF] = 0;
        // if self.registers[y] > self.registers[x] {
        //     self.registers[0xF] = 1;
        // }
        // self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);
        // self.increment_program_counter(1);

        let difference = self.registers[y].wrapping_sub(self.registers[x]);
        self.registers[0xF] = if self.registers[y] >= self.registers[x] {
            1
        } else {
            0
        };
        if x != 0xF {
            self.registers[x] = difference;
        };
        self.increment_program_counter(1);
    }

    // SHL Vx {, Vy}
    #[allow(non_snake_case)]
    fn opcode_8xyE(&mut self, x: usize, y: usize) {
        // Set Vx = Vx SHL 1.

        // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0.
        // Then Vx is multiplied by 2.

        let original_vx = self.registers[x];

        if self.shift_quirk_vx_eq_vy {
            self.registers[x] = self.registers[y];
        }

        // gemini code
        // let vx_value_before_shift = self.registers[x];

        // self.registers[0xF] = if (vx_value_before_shift & 0x80) != 0 {
        //     1
        // } else {
        //     0
        // };
        // self.registers[x] <<= 1;
        // self.increment_program_counter(1);

        if original_vx & 0x80 > 0 {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
        // self.registers[x] &= 0x7F;
        if x != 0xF {
            self.registers[x] <<= 1;
        }
        self.increment_program_counter(1);

        // my original code
        // self.registers[0xF] = 0;
        // if self.registers[x] > 127 {
        //     self.registers[0xF] = 1;
        // }
        // self.registers[x] = self.registers[x].wrapping_mul(2);
        // self.increment_program_counter(1);
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
    #[allow(non_snake_case)]
    fn opcode_Annn(&mut self, nnn: u16) {
        // Set I = nnn.

        // The value of register I is set to nnn.

        self.i = nnn;
        self.increment_program_counter(1);
    }

    // JP V0, addr
    #[allow(non_snake_case)]
    fn opcode_Bnnn(&mut self, nnn: u16) {
        // Jump to location nnn + V0.

        // The program counter is set to nnn plus the value of V0.

        self.program_counter = nnn + self.registers[0] as u16;
    }

    // RND Vx, byte
    #[allow(non_snake_case)]
    fn opcode_Cxkk(&mut self, x: usize, kk: u8) {
        // Set Vx = random byte AND kk.

        // The interpreter generates a random number from 0 to 255,
        // which is then ANDed with the value kk. The results are stored in Vx.
        let mut rng = rand::thread_rng();
        let num = rng.gen_range(0..256) as u8;
        self.registers[x] = num & kk;
        self.increment_program_counter(1);
    }

    // DRW Vx, Vy, nibble
    #[allow(non_snake_case)]
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

        self.registers[0xF] = 0;
        // let cx: usize = self.registers[x] as usize;
        // let cy: usize = self.registers[y] as usize;

        // for byte in 0..n {
        //     // do stuff
        //     for bit_shift in 0..8 {
        //         // do something with the bits
        //         let bit = (self.ram[self.i as usize + byte] >> (7 - bit_shift)) & 0x1;
        //         let cx = (x + bit_shift) % 32;
        //         let cy = (y + byte) % 64;
        //         if self.screen[cy][cx] == 1 && bit == 1 {
        //             self.screen[cy][cx] = 0;
        //             self.registers[0xF] = 1;
        //         } else {
        //             self.screen[cy][cx] = bit;
        //         }
        //         println!("{}", self.screen[cy][cx]);
        //     }
        // }

        for byte in 0..n {
            let cy = (self.registers[y] as usize + byte) % 32;
            for bit in 0..8 {
                let cx = (self.registers[x] as usize + bit) % 64;
                let color = (self.ram[self.i as usize + byte] >> (7 - bit)) & 1;
                self.registers[0xF] |= color & self.screen[cy][cx];
                self.screen[cy][cx] ^= color;

                // println!("X: {}, Y: {}, Val: {}", cx, cy, self.screen[cy][cx]);
            }
        }

        // self.screen[0][0] = 1;
        // println!("{}", self.screen[0][0]);
        self.screen_dirty = true;
        self.increment_program_counter(1);
    }

    // SKP Vx
    #[allow(non_snake_case)]
    fn opcode_Ex9E(&mut self, x: usize) {
        // Skip next instruction if key with the value of Vx is pressed.

        // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position,
        // PC is increased by 2.

        let mut matching_key_pressed = false;
        for key in self.keyboard.iter() {
            if self.registers[x] == key.clone() {
                matching_key_pressed = true;
            }
        }

        if matching_key_pressed {
            self.increment_program_counter(2);
        } else {
            self.increment_program_counter(1);
        }
    }

    // SKNP A1
    #[allow(non_snake_case)]
    fn opcode_ExA1(&mut self, x: usize) {
        // Skip next instruction if key with the value of Vx is not pressed.

        // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.

        let mut matching_key_pressed = false;
        for key in self.keyboard.iter() {
            if self.registers[x] == key.clone() {
                matching_key_pressed = true;
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
    fn opcode_Fx0A(&mut self, x: usize) {
        // Wait for a key press, store the value of the key in Vx.

        // All execution stops until a key is pressed, then the value of that key is stored in Vx.

        if !self.keyboard.is_empty() {
            self.registers[x] = self.keyboard[self.keyboard.len() - 1];
            self.increment_program_counter(1);
        }
    }

    // LD DT, Vx
    #[allow(non_snake_case)]
    fn opcode_Fx15(&mut self, x: usize) {
        // Set delay timer = Vx.

        // DT is set equal to the value of Vx.

        self.delay_timer = self.registers[x];
        self.increment_program_counter(1);
    }

    // LD ST, Vx
    #[allow(non_snake_case)]
    fn opcode_Fx18(&mut self, x: usize) {
        // Set sound timer = Vx.

        // ST is set equal to the value of Vx.

        self.sound_timer = self.registers[x];
        self.increment_program_counter(1);
    }

    // ADD I, Vx
    #[allow(non_snake_case)]
    fn opcode_Fx1E(&mut self, x: usize) {
        // Set I = I + Vx.

        // The values of I and Vx are added, and the results are stored in I.

        self.i = self.i + self.registers[x] as u16;
        self.increment_program_counter(1);
    }

    // LD F, Vx
    #[allow(non_snake_case)]
    fn opcode_Fx29(&mut self, x: usize) {
        // Set I = location of sprite for digit Vx.

        // The value of I is set to the location in memory for the hexadecimal sprite corresponding to the value of Vx.
        // See section 2.4, Display, for more information on the Chip-8 hexadecimal font.

        // my code
        // self.i = self.ram[x * 5] as u16;
        // self.increment_program_counter(1);

        // gemini suggestion below
        self.i = self.registers[x] as u16 * 5;
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
        self.increment_program_counter(1);
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
