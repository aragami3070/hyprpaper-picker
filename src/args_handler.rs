use std::error::Error;

use crate::{
    choose::{next_wallpaper, prev_wallpaper, random_wallpaper},
    dir_scan::{get_all_wallpapers, Args, CliCommand},
    hyprctl::{set_new_wallpaper, ActiveWallpaper},
};

pub fn handler(args: Args, active_wallpaper: ActiveWallpaper) -> Result<(), Box<dyn Error>> {
    match args.command {
        CliCommand::Rand { dir_path } => {
            let wallpapers = get_all_wallpapers(dir_path)?;

            let new_wallpaper = random_wallpaper(wallpapers, active_wallpaper)?;

            match set_new_wallpaper(new_wallpaper) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }

        CliCommand::Next { dir_path } => {
            let mut wallpapers = get_all_wallpapers(dir_path)?;

            wallpapers.sort_by_key(|a| a.path.0.clone());

            let new_wallpaper = next_wallpaper(wallpapers, active_wallpaper)?;

            match set_new_wallpaper(new_wallpaper) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }

        CliCommand::Prev { dir_path } => {
            let mut wallpapers = get_all_wallpapers(dir_path)?;

            wallpapers.sort_by_key(|a| a.path.0.clone());

            let new_wallpaper = prev_wallpaper(wallpapers, active_wallpaper)?;

            match set_new_wallpaper(new_wallpaper) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        }
    }
}
