use crate::{
    args::{Args, CliCommand},
    choose::{next_wallpaper, prev_wallpaper, random_wallpaper},
    dir_scan::get_all_wallpapers,
    errors::HyprpaperPickerError,
};

pub fn handler(args: Args) -> Result<(), HyprpaperPickerError> {
    match args.command {
        CliCommand::Setup {
            wallpaper_path,
            monitor,
        } => {
            todo!()
        }
        CliCommand::Rand { dir_path } => {
            let wallpapers = get_all_wallpapers(dir_path)?;

            todo!();
        }

        CliCommand::Next { dir_path } => {
            let mut wallpapers = get_all_wallpapers(dir_path)?;

            wallpapers.sort_by_key(|a| a.path.0.clone());

            todo!();
        }

        CliCommand::Prev { dir_path } => {
            let mut wallpapers = get_all_wallpapers(dir_path)?;

            wallpapers.sort_by_key(|a| a.path.0.clone());
            todo!()
        }
    }
}
