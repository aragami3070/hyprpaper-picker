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

/// Get cur wallpaper from config file
pub fn get_cur_wallpaper(monitor: Option<Monitor>) -> Result<Wallpaper, HyprpaperPickerError> {
    let config_buf = create_if_not_exists()?;

    let mut cur_wallpaper: Wallpaper = if config_buf.exists() {
        let wallpaper_info = fs::read_to_string(&config_buf)?;
        toml::from_str(&wallpaper_info)?
    } else {
        return Err(HyprpaperPickerError::Config(String::from(
            "wallpaper or/and monitor didn't setup in config",
        )));
    };

    if let Some(monit) = monitor {
        cur_wallpaper.monitor = monit;
    }

    Ok(cur_wallpaper)
}
