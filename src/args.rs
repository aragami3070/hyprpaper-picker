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
    /// Setup monitor and wallpaper
    Setup {
        #[clap(long, short)]
        wallpaper_path: Path,
        #[clap(long, short)]
        monitor: Monitor,
    },
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
