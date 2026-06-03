mod args;
mod args_handler;
mod choose;
mod dir_scan;
mod hyprctl;

use clap::Parser;
use std::process;

use crate::{args::Args, args_handler::handler};

fn main() {
    let args: Args = Args::parse();
    if let Err(err) =
        hyprctl::get_active_wallpaper().and_then(|active_wallpaper| handler(args, active_wallpaper))
    {
        eprintln!("Error: {err}");
        process::exit(1)
    };
}
