use crate::{
    args::{Args, CliCommand},
    choose::{next_wallpaper, prev_wallpaper, random_wallpaper},
    config::{get_cur_wallpaper, set_cur_wallpaper},
    dir_scan::get_all_wallpapers,
    errors::HyprpaperPickerError,
    hyprctl::{Wallpaper, is_wallpaper_path, set_new_wallpaper},
};

pub fn handler(args: Args) -> Result<(), HyprpaperPickerError> {
    match args.command {
        CliCommand::DisplayLast { monitor } => {
            todo!()
        }
        CliCommand::Setup {
            wallpaper_path,
            monitor,
        } => {
            if !is_wallpaper_path(&wallpaper_path.0) {
                return Err(HyprpaperPickerError::Input(
                    "wallpaper file must be: png, jpg, jpeg or jxl".to_string(),
                ));
            }
            let monitor = if let Some(monit) = monitor {
                monit
            } else {
                let Wallpaper { monitor: monit, .. } = get_cur_wallpaper(monitor)?;
                monit
            };

            let wallpaper = Wallpaper {
                path: wallpaper_path,
                monitor,
            };

            set_new_wallpaper(&wallpaper)?;
            set_cur_wallpaper(&wallpaper)
        }
        CliCommand::Rand { dir_path, monitor } => {
            let wallpapers = get_all_wallpapers(dir_path)?;

            todo!();
        }

        CliCommand::Next { dir_path, monitor } => {
            let mut wallpapers = get_all_wallpapers(dir_path)?;

            wallpapers.sort_by_key(|a| a.path.0.clone());

            todo!();
        }

        CliCommand::Prev { dir_path, monitor } => {
            let mut wallpapers = get_all_wallpapers(dir_path)?;

            wallpapers.sort_by_key(|a| a.path.0.clone());
            todo!()
        }
    }
}
