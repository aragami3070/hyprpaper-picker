use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::hyprctl::Monitor;

#[derive(Parser)]
#[command(version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: CliCommand,
}

#[derive(Subcommand)]
pub enum CliCommand {
    /// Display wallpaper from config
    DisplayLast {
        #[clap(long, short)]
        monitor: Option<Monitor>,
    },

    /// Setup monitor and wallpaper
    Setup {
        #[clap(long, short)]
        wallpaper_path: PathBuf,
        #[clap(long, short)]
        monitor: Option<Monitor>,
    },

    /// Get next wallpaper
    Next {
        #[clap(long, short)]
        dir_path: Option<PathBuf>,
        #[clap(long, short)]
        monitor: Option<Monitor>,
    },

    /// Get prev wallpaper
    Prev {
        #[clap(long, short)]
        dir_path: Option<PathBuf>,
        #[clap(long, short)]
        monitor: Option<Monitor>,
    },

    /// Get random wallpaper
    Rand {
        #[clap(long, short)]
        dir_path: Option<PathBuf>,
        #[clap(long, short)]
        monitor: Option<Monitor>,
    },
}
