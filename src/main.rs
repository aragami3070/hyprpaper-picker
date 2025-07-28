mod choose;
mod dir_scan;
mod hyprctl;

use clap::Parser;
use std::process;

use crate::dir_scan::{Args, CliCommand, get_all_wallpapers};

fn main() {
    let args: Args = Args::parse();
    let active_wallpaper = match hyprctl::get_active_wallpaper() {
        Ok(w) => w,
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    };

    println!("Result: {active_wallpaper:?}");

    match args.command {
        CliCommand::Rand { dir_path } => {
            let wallpapers = match get_all_wallpapers(dir_path) {
                Ok(w) => w,
                Err(err) => {
                    eprintln!("Error: {err}");
                    process::exit(1);
                }
            };

            for wallpaper in wallpapers {
                println!("{wallpaper:?}")
            }
        }

        CliCommand::Next { dir_path } => {
            let mut wallpapers = match get_all_wallpapers(dir_path) {
                Ok(w) => w,
                Err(err) => {
                    eprintln!("Error: {err}");
                    process::exit(1);
                }
            };

            wallpapers.sort_by_key(|a| a.path.0.clone());

            for wallpaper in wallpapers {
                println!("{wallpaper:?}")
            }
        }
    }
}
