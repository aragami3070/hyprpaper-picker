use std::{error::Error, fmt, process::Command};

pub struct Wallpaper {
    path: String,
}

pub struct ActiveWallpaper(pub Wallpaper);

#[derive(Debug)]
pub struct HyprpaperError {
    description: String,
}

impl fmt::Display for HyprpaperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for HyprpaperError {}

#[derive(Debug)]
enum ErrorVariant {
    ListActive(HyprpaperError),
}

#[derive(Debug)]
pub struct HyprctlError {
    variant: ErrorVariant,
}

impl Error for HyprctlError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.variant {
            ErrorVariant::ListActive(er) => Some(er),
        }
    }
}

impl fmt::Display for HyprctlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.variant {
            ErrorVariant::ListActive(_) => {
                write!(f, "run hyprctl hyprpaper listactive error received")
            }
        }
    }
}

