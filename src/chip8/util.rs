use bevy::prelude::*;

pub fn keycode_to_hex(keycode: KeyCode) -> u8 {
    match keycode {
        KeyCode::Key1 => 0x1,
        KeyCode::Key2 => 0x2,
        KeyCode::Key3 => 0x3,
        KeyCode::Key4 => 0xC,
        KeyCode::Q => 0x4,
        KeyCode::W => 0x5,
        KeyCode::E => 0x6,
        KeyCode::R => 0xD,
        KeyCode::A => 0x7,
        KeyCode::S => 0x8,
        KeyCode::D => 0x9,
        KeyCode::F => 0xE,
        KeyCode::Z => 0xA,
        KeyCode::X => 0x0,
        KeyCode::C => 0xB,
        KeyCode::V => 0xF,
        _ => 0xFF, // Returns 255 in case of one of the non-allowed keys are pressed
    }
}
