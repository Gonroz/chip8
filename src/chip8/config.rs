use bevy::prelude::*;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use toml;

// use crate::chip8::theme::Theme;

#[derive(Deserialize, Resource, Clone)]
pub struct Config {
    pub rom: String,
    pub theme: String,
    pub instructions_per_second: u32,
    pub pitch_frequency: f32,
    pub pitch_duration: u64,
}

impl Config {
    pub fn new() -> Self {
        return get_config();
    }

    pub fn get_rom_path(&self) -> String {
        // Start the path based off where cargo run is done
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")
            .expect("CARGO_MANIFEST_DIR not set. Are you running this via cargo?");
        let path = PathBuf::from(manifest_dir);

        // Construct the absolute path to the config.toml
        let mut rom_folder = path.clone();
        rom_folder.push("roms");

        let mut rom_path = rom_folder.clone();
        // rom_path.push("{self.ram}.toml");
        let rom: &str = &self.rom;
        rom_path.push(format!("{rom}.ch8"));

        return rom_path
            .to_str()
            .expect("Failed to convert the following path to a string: {rom_path}")
            .to_string();
    }
}

pub fn get_config() -> Config {
    // Start the path based off where cargo run is done
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set. Are you running this via cargo?");
    let path = PathBuf::from(manifest_dir);

    // Construct the absolute path to the config.toml
    let mut config_path = path.clone();
    config_path.push("config.toml");

    // Read the file to get the toml as a string
    let config_file_contents = fs::read_to_string(&config_path).expect(&format!(
        "Can't find config at path: {}",
        config_path.display()
    ));

    // construct a Config based off the toml contents
    let config: Config =
        toml::from_str(&config_file_contents).expect("Failed to deserialize the config toml file.");
    return config;
}

#[cfg(test)]
mod tests {
    use super::*;

    // const TEST_TOML: &str = r"  foreground = [163, 114, 28]
    //                             background = [52, 48, 56]";

    #[test]
    fn test_config() {
        let config: Config = Config::new();
        assert_eq!(config.rom, "test.ch8");
        assert_eq!(config.theme, "test");
    }
}
