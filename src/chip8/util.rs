use bevy::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn keycode_to_hex(keycode: &KeyCode) -> u8 {
    match keycode {
        KeyCode::Digit1 => 0x1,
        KeyCode::Digit2 => 0x2,
        KeyCode::Digit3 => 0x3,
        KeyCode::Digit4 => 0xC,
        KeyCode::KeyQ => 0x4,
        KeyCode::KeyW => 0x5,
        KeyCode::KeyE => 0x6,
        KeyCode::KeyR => 0xD,
        KeyCode::KeyA => 0x7,
        KeyCode::KeyS => 0x8,
        KeyCode::KeyD => 0x9,
        KeyCode::KeyF => 0xE,
        KeyCode::KeyZ => 0xA,
        KeyCode::KeyX => 0x0,
        KeyCode::KeyC => 0xB,
        KeyCode::KeyV => 0xF,
        _ => 0xFF, // Returns 255 in case of one of the non-registered keys are pressed
    }
}

pub fn get_rom_to_load() -> String {
    // Get the path to the project root (where Cargo.toml is)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set. Are you running this via cargo?");
    let mut path = PathBuf::from(manifest_dir);

    // Construct the absolute path to rom_to_load.txt (in the root)
    let mut rom_to_load_path = path.clone();
    rom_to_load_path.push("rom_to_load.txt");

    // Read the file to get the ROM filename
    let rom_name = fs::read_to_string(&rom_to_load_path).expect(&format!(
        "Can't read rom_to_load.txt at path: {}",
        rom_to_load_path.display()
    ));
    let rom_filename = rom_name.trim();

    // Construct the full path
    path.push("roms");
    path.push(rom_filename);
    let final_path = path
        .to_str()
        .expect("Invalid path construction")
        .to_string();
    println!("Loading ROM from fixed absolute path: {}", final_path);

    return final_path;
}

pub const CHIP8_FONT: [u8; 80] = [
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
