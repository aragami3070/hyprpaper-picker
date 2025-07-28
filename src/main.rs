mod choose;
mod dir_scan;
mod hyprctl;

use clap::Parser;
use std::process;

use crate::{choose::random_wallpaper, dir_scan::{get_all_wallpapers, Args, CliCommand}};

fn main() {
    let args: Args = Args::parse();
    let active_wallpaper = match hyprctl::get_active_wallpaper() {
        Ok(w) => w,
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    };

    println!("Active wallpaper: {active_wallpaper:?}");

    match args.command {
        CliCommand::Rand { dir_path } => {
            let wallpapers = match get_all_wallpapers(dir_path) {
                Ok(w) => w,
                Err(err) => {
                    eprintln!("Error: {err}");
                    process::exit(1);
                }
            };

			let new_wallpaper = random_wallpaper(wallpapers, active_wallpaper);

			println!("Result: {new_wallpaper:?}");
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
