use crate::{
    args::{Args, CliCommand},
    choose::{next_wallpaper, prev_wallpaper, random_wallpaper},
    config::{get_cur_wallpaper, set_cur_wallpaper},
    dir_scan::get_all_wallpapers,
    errors::HyprpaperPickerError,
    hyprctl::{ActiveWallpaper, Wallpaper, is_wallpaper_path, set_new_wallpaper},
};

pub fn handler(args: Args) -> Result<(), HyprpaperPickerError> {
    match args.command {
        CliCommand::DisplayLast { monitor } => {
            let wallpaper = get_cur_wallpaper(monitor)?;
            set_new_wallpaper(&wallpaper)?;
            set_cur_wallpaper(&wallpaper)
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
            let (dir_path, monitor) = if let Some(dir_p) = dir_path
                && let Some(monit) = monitor
            {
                let dir_p = dir_p.to_dir();
                (dir_p, monit)
            } else {
                let Wallpaper {
                    monitor: monit,
                    path,
                } = get_cur_wallpaper(monitor)?;
                let dir_p = path.to_dir();
                (dir_p, monit)
            };

            let wallpapers = get_all_wallpapers(dir_path.clone())?;

            let Wallpaper { path, monitor } = get_cur_wallpaper(Some(monitor))?;

            let wallpaper =
                random_wallpaper(wallpapers, ActiveWallpaper(Wallpaper { path, monitor }));

            set_new_wallpaper(&wallpaper)?;
            set_cur_wallpaper(&wallpaper)
        }

        CliCommand::Next { dir_path, monitor } => {
            todo!();
        }

        CliCommand::Prev { dir_path, monitor } => {
            todo!()
        }
    }
}
