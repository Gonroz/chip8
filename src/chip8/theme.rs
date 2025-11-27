use bevy::prelude::*;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use toml;

#[derive(Deserialize, Resource, Clone)]
pub struct Theme {
    pub foreground: [u8; 3],
    pub background: [u8; 3],
}

impl Theme {
    pub fn new(theme_name: &str) -> Self {
        return get_theme(theme_name);
    }
}

pub fn get_theme(theme_name: &str) -> Theme {
    // Start the path based off where cargo run is done
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR not set. Are you running this via cargo?");
    let path = PathBuf::from(manifest_dir);

    // Construct the absolute path to the chosen theme
    let mut theme_path = path.clone();
    theme_path.push("themes");
    theme_path.push(format!("{}{}", theme_name, ".toml"));

    // Read the file to get the toml as a string
    let theme_file_contents = fs::read_to_string(&theme_path).expect(&format!(
        "Can't read theme at path: {}",
        theme_path.display()
    ));

    // construct a Theme based off the toml contents
    let theme: Theme =
        toml::from_str(&theme_file_contents).expect("Failed to deserialize the theme's toml file.");
    return theme;
}

#[cfg(test)]
mod tests {
    use super::*;

    // const TEST_TOML: &str = r"  foreground = [163, 114, 28]
    //                             background = [52, 48, 56]";

    #[test]
    fn test_theme() {
        let theme: Theme = get_theme("test");
        // foreground
        assert_eq!(theme.foreground[0], 163 as u8);
        assert_eq!(theme.foreground[1], 114 as u8);
        assert_eq!(theme.foreground[2], 28 as u8);
        // background
        assert_eq!(theme.background[0], 52 as u8);
        assert_eq!(theme.background[1], 48 as u8);
        assert_eq!(theme.background[2], 56 as u8);
    }
}
