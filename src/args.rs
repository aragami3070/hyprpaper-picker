use clap::{Parser, Subcommand};

use crate::hyprctl::Path;

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
