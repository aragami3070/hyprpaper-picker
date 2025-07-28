use std::error::Error;

use crate::hyprctl::{ActiveWallpaper, NewWallpaper, Wallpaper};

pub fn random_wallpaper(
    mut wallpapers: Vec<Wallpaper>,
    active_wallpaper: ActiveWallpaper,
) -> Result<NewWallpaper, Box<dyn Error>> {
    let remove_active_w = active_wallpaper.0.clone();
    wallpapers.retain(|x| (*x).path != remove_active_w.path);

    let rand_num = rand::random_range(0..wallpapers.len() - 1);

    let new_wallpaper = NewWallpaper(Wallpaper {
        path: wallpapers[rand_num].path.to_owned(),
        monitor: active_wallpaper.0.monitor,
    });

    Ok(new_wallpaper)
}
