use std::{fs, path::PathBuf};

use crate::{
    errors::HyprpaperPickerError,
    hyprctl::{Monitor, Wallpaper, is_wallpaper_path},
};

/// Get all path to files from dir path
pub fn get_all_wallpapers(dir_path: PathBuf) -> Result<Vec<Wallpaper>, HyprpaperPickerError> {
    let paths = fs::read_dir(dir_path)?;

    let mut wallpapers = Vec::new();

    for wallp_path in paths {
        let path = wallp_path?.path();
        if path.is_file() && is_wallpaper_path(&path) {
            let wallpaper = Wallpaper {
                path,
                monitor: Monitor(String::new()),
            };
            wallpapers.push(wallpaper);
        }
    }

    Ok(wallpapers)
}
