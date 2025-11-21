use bevy::prelude::*;

mod chip8;

fn main() {
    App::new().add_plugins(chip8::Chip8Plugin).run();
}
