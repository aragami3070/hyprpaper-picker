use std::path::PathBuf;

use crate::{
    args::{Args, CliCommand},
    choose::{next_wallpaper, prev_wallpaper, random_wallpaper},
    config::{get_cur_wallpaper, set_cur_wallpaper},
    dir_scan::get_all_wallpapers,
    errors::HyprpaperPickerError,
    hyprctl::{Monitor, Wallpaper, is_wallpaper_path, set_new_wallpaper},
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
        } => setup_handler(wallpaper_path, monitor),

        CliCommand::Rand { dir_path, monitor } => choose_wallpaper_handler(
            dir_path,
            monitor,
            |active_wallpaper: Wallpaper, wallpapers: Vec<Wallpaper>| -> Wallpaper {
                random_wallpaper(wallpapers, active_wallpaper)
            },
        ),

        CliCommand::Next { dir_path, monitor } => choose_wallpaper_handler(
            dir_path,
            monitor,
            |active_wallpaper: Wallpaper, wallpapers: Vec<Wallpaper>| -> Wallpaper {
                next_wallpaper(&wallpapers, active_wallpaper)
            },
        ),
        CliCommand::Prev { dir_path, monitor } => choose_wallpaper_handler(
            dir_path,
            monitor,
            |active_wallpaper: Wallpaper, wallpapers: Vec<Wallpaper>| -> Wallpaper {
                prev_wallpaper(&wallpapers, active_wallpaper)
            },
        ),
    }
}

fn choose_wallpaper_handler<F: Fn(Wallpaper, Vec<Wallpaper>) -> Wallpaper>(
    dir_path: Option<PathBuf>,
    monitor: Option<Monitor>,
    choose_clouser: F,
) -> Result<(), HyprpaperPickerError> {
    let (dir_path, monitor) = setup_dir_path_and_monitor(dir_path, monitor)?;

    let active_wallpaper = get_cur_wallpaper(Some(monitor))?;
    let mut wallpapers = get_all_wallpapers(dir_path.clone())?;
    wallpapers.sort_by(|a, b| a.path.cmp(&b.path));

    let wallpaper = choose_clouser(active_wallpaper, wallpapers);

    set_new_wallpaper(&wallpaper)?;
    set_cur_wallpaper(&wallpaper)
}

fn setup_handler(
    wallpaper_path: PathBuf,
    monitor: Option<Monitor>,
) -> Result<(), HyprpaperPickerError> {
    if !is_wallpaper_path(&wallpaper_path) {
        return Err(HyprpaperPickerError::Input(
            "wallpaper file must be: png, jpg, jpeg or jxl".to_string(),
        ));
    }

    let monitor = if let Some(monit) = monitor {
        monit
    } else {
        get_cur_wallpaper(monitor)?.monitor
    };

    let wallpaper = Wallpaper {
        path: wallpaper_path,
        monitor,
    };

    set_new_wallpaper(&wallpaper)?;
    set_cur_wallpaper(&wallpaper)
}

fn to_dir(path: &std::path::Path) -> PathBuf {
    if is_wallpaper_path(path) {
        path.parent().unwrap_or(path).to_path_buf()
    } else {
        path.to_path_buf()
    }
}

fn setup_dir_path_and_monitor(
    inp_dir_path: Option<PathBuf>,
    inp_monitor: Option<Monitor>,
) -> Result<(PathBuf, Monitor), HyprpaperPickerError> {
    let (dir_path, monitor) = if let Some(dir_path) = &inp_dir_path
        && let Some(monitor) = inp_monitor
    {
        (to_dir(dir_path), monitor)
    } else {
        let Wallpaper { monitor, path } = get_cur_wallpaper(inp_monitor)?;

        let dir_path = if let Some(dir_p) = inp_dir_path {
            to_dir(&dir_p)
        } else {
            to_dir(&path)
        };

        (dir_path, monitor)
    };

    Ok((dir_path, monitor))
}
