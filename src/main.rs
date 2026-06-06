mod args;
mod args_handler;
mod choose;
mod config;
mod dir_scan;
mod errors;
mod hyprctl;

use clap::Parser;
use std::process;

use crate::{args::Args, args_handler::handler};

fn main() {
    let args: Args = Args::parse();
    if let Err(err) = handler(args) {
        eprintln!("Error: {err}");
        process::exit(1)
    };
}
