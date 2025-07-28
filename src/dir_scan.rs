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
    Next { dir_path: Path },

    /// Get random wallpaper wallpaper
    Rand { dir_path: Path },
}
