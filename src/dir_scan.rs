use std::{fs, io};

use crate::{
    errors::HyprpaperPickerError,
    hyprctl::{Monitor, Path, Wallpaper, is_wallpaper_path},
};

/// Get all path to files from dir path
pub fn get_all_wallpapers(dir_path: Path) -> Result<Vec<Wallpaper>, HyprpaperPickerError> {
    let paths = fs::read_dir(dir_path.0)?;

    let mut wallpapers = Vec::new();

    for wallp_path in paths {
        let path_pars = match wallp_path?.path().to_str() {
            Some(path) if is_wallpaper_path(path) => path.to_string(),
            Some(_) => continue,
            None => {
                return Err(HyprpaperPickerError::Io(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "can't parse path to wallpaper",
                )));
            }
        };

        let wallpaper = Wallpaper {
            path: Path(path_pars),
            monitor: Monitor(String::new()),
        };
        wallpapers.push(wallpaper);
    }

    Ok(wallpapers)
}
