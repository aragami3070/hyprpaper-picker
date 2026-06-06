use std::{fs, path::PathBuf};

use crate::{
    errors::HyprpaperPickerError,
    hyprctl::{Monitor, Wallpaper},
};

/// Create config file by path ~/.config/hyprpaper_picker/config.toml if not
/// exists and return config buf
fn create_if_not_exists() -> Result<PathBuf, HyprpaperPickerError> {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    let config_dir = home_dir.join(".config").join("hyprpaper_picker");
    let config_buf = config_dir.join("config.toml");

    fs::create_dir_all(&config_dir)?;
    Ok(config_buf)
}
