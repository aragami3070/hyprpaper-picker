mod args_handler;
mod choose;
mod dir_scan;
mod hyprctl;

use clap::Parser;
use std::process;

use crate::{args_handler::handler, dir_scan::Args};

fn main() {
    let args: Args = Args::parse();
    let active_wallpaper = match hyprctl::get_active_wallpaper() {
        Ok(w) => w,
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    };

    match handler(args, active_wallpaper) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    }
}
