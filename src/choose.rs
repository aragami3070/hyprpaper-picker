use std::error::Error;

use crate::hyprctl::{ActiveWallpaper, NewWallpaper, Wallpaper};

/// Get random NewWallpaper from Wallpaper vec
pub fn random_wallpaper(
    mut wallpapers: Vec<Wallpaper>,
    active_wallpaper: ActiveWallpaper,
) -> Result<NewWallpaper, Box<dyn Error>> {
    wallpapers.retain(|x| x.path != active_wallpaper.0.path);

    let rand_num = rand::random_range(0..wallpapers.len() - 1);

    let new_wallpaper = NewWallpaper(Wallpaper {
        path: wallpapers[rand_num].path.to_owned(),
        monitor: active_wallpaper.0.monitor,
    });

    Ok(new_wallpaper)
}

/// Get next NewWallpaper from Wallpaper vec
pub fn next_wallpaper(
    wallpapers: Vec<Wallpaper>,
    active_wallpaper: ActiveWallpaper,
) -> Result<NewWallpaper, Box<dyn Error>> {
    let active_wallp_index = wallpapers
        .iter()
        .position(|w| w.path == active_wallpaper.0.path);

    let new_wallpaper = match active_wallp_index {
        Some(i) => {
            if let Some(value) = wallpapers.get(i + 1) {
                value.to_owned()
            } else {
                wallpapers[0].to_owned()
            }
        }
        None => wallpapers[0].to_owned(),
    };

    Ok(NewWallpaper(Wallpaper {
        path: new_wallpaper.path,
        monitor: active_wallpaper.0.monitor,
    }))
}

pub fn prev_wallpaper(
    wallpapers: Vec<Wallpaper>,
    active_wallpaper: ActiveWallpaper,
) -> Result<NewWallpaper, Box<dyn Error>> {
    let active_wallp_index = wallpapers
        .iter()
        .position(|w| (*w).path == active_wallpaper.0.path);

    let new_wallpaper = match active_wallp_index {
        Some(i) => {
            if i > 0 && let Some(value) = wallpapers.get(i - 1) {
                value.to_owned()
            } else {
                wallpapers[wallpapers.len() - 1].to_owned()
            }
        }
        None => wallpapers[wallpapers.len() - 1].to_owned(),
    };

    Ok(NewWallpaper(Wallpaper {
        path: new_wallpaper.path,
        monitor: active_wallpaper.0.monitor,
    }))
}
