use crate::{
    args::{Args, CliCommand},
    choose::{next_wallpaper, prev_wallpaper, random_wallpaper},
    config::{get_cur_wallpaper, set_cur_wallpaper},
    dir_scan::get_all_wallpapers,
    errors::HyprpaperPickerError,
    hyprctl::{ActiveWallpaper, Monitor, Path, Wallpaper, is_wallpaper_path, set_new_wallpaper},
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
            let (dir_path, monitor) = setup_dir_path_and_monitor(dir_path, monitor)?;

            let wallpapers = get_all_wallpapers(dir_path.clone())?;

            let Wallpaper { path, monitor } = get_cur_wallpaper(Some(monitor))?;

            let wallpaper =
                random_wallpaper(wallpapers, ActiveWallpaper(Wallpaper { path, monitor }));

            set_new_wallpaper(&wallpaper)?;
            set_cur_wallpaper(&wallpaper)
        }

        CliCommand::Next { dir_path, monitor } => {
            let (dir_path, monitor) = setup_dir_path_and_monitor(dir_path, monitor)?;

            let mut wallpapers = get_all_wallpapers(dir_path.clone())?;

            let Wallpaper { path, monitor } = get_cur_wallpaper(Some(monitor))?;

            wallpapers.sort_by_key(|a| a.path.0.clone());

            let wallpaper =
                next_wallpaper(wallpapers, ActiveWallpaper(Wallpaper { path, monitor }));

            set_new_wallpaper(&wallpaper)?;
            set_cur_wallpaper(&wallpaper)
        }

        CliCommand::Prev { dir_path, monitor } => {
            let (dir_path, monitor) = setup_dir_path_and_monitor(dir_path, monitor)?;

            let mut wallpapers = get_all_wallpapers(dir_path.clone())?;

            let Wallpaper { path, monitor } = get_cur_wallpaper(Some(monitor))?;

            wallpapers.sort_by_key(|a| a.path.0.clone());

            let wallpaper =
                prev_wallpaper(wallpapers, ActiveWallpaper(Wallpaper { path, monitor }));

            set_new_wallpaper(&wallpaper)?;
            set_cur_wallpaper(&wallpaper)
        }
    }
}

fn setup_dir_path_and_monitor(
    inp_dir_path: Option<Path>,
    inp_monitor: Option<Monitor>,
) -> Result<(Path, Monitor), HyprpaperPickerError> {
    let (dir_path, monitor) = if let Some(dir_path) = &inp_dir_path
        && let Some(monitor) = inp_monitor
    {
        let dir_path = dir_path.to_dir();
        (dir_path, monitor)
    } else {
        let Wallpaper { monitor, path } = get_cur_wallpaper(inp_monitor)?;

        let dir_path = if let Some(dir_p) = inp_dir_path {
            dir_p
        } else {
            path.to_dir()
        };

        (dir_path, monitor)
    };

    Ok((dir_path, monitor))
}
