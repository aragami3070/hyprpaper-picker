use clap::{Parser, Subcommand};

use crate::hyprctl::{Monitor, Path};

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
        wallpaper_path: Path,
        #[clap(long, short)]
        monitor: Option<Monitor>,
    },

    /// Get next wallpaper
    Next {
        #[clap(long, short)]
        dir_path: Path,
        #[clap(long, short)]
        monitor: Option<Monitor>,
    },

    /// Get prev wallpaper
    Prev {
        #[clap(long, short)]
        dir_path: Path,
        #[clap(long, short)]
        monitor: Option<Monitor>,
    },

    /// Get random wallpaper
    Rand {
        #[clap(long, short)]
        dir_path: Path,
        #[clap(long, short)]
        monitor: Option<Monitor>,
    },
}
