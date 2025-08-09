use std::{error::Error, fs, io};

use clap::{Parser, Subcommand};

use crate::hyprctl::{Monitor, Path, Wallpaper};

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Get next wallpaper
    Next {
        #[clap(long, short)]
        dir_path: Path,
    },

    /// Get prev wallpaper
    Prev {
        #[clap(long, short)]
        dir_path: Path,
    },

    /// Get random wallpaper
    Rand {
        #[clap(long, short)]
        dir_path: Path,
    },
}

/// Get all path to files from dir path
pub fn get_all_wallpapers(dir_path: Path) -> Result<Vec<Wallpaper>, Box<dyn Error>> {
    let paths = fs::read_dir(dir_path.0)?;

    let mut wallpapers = Vec::new();

    for wallp_path in paths {
        let path_pars = match wallp_path?.path().to_str() {
            Some(path) => path.to_string(),
            None => {
                return Err(Box::new(io::Error::new(
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
