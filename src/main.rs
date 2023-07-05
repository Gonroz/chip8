use bevy::prelude::*;

mod chip8;

fn main() {
    App::new().add_plugin(chip8::Chip8Plugin).run();
}
