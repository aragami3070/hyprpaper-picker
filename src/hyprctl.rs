use serde::Deserialize;
use std::{error::Error, fmt, process::Command, str::FromStr};

use crate::errors::HyprpaperPickerError;

/// Path to dir or file
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Path(pub String);

impl FromStr for Path {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err("Path cannot be empty".to_string())
        } else if !s.contains('/') {
            Err("Path must have '/'".to_string())
        } else if !is_wallpaper_path(s) {
            Err("File can be '.png', '.jpg', '.jpeg', '.jxl'".to_string())
        } else {
            Ok(Path(s.to_string()))
        }
    }
}

/// Monitor port (for example DP-2 or eDP-1)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Monitor(pub String);

/// Wallpaper info
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Wallpaper {
    pub path: Path,
    pub monitor: Monitor,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActiveWallpaper(pub Wallpaper);

/// Errors returned by the hyprctl
#[derive(Debug, PartialEq, Eq)]
pub struct HyprctlError {
    kind: HyprctlErrorKind,
    description: String,
}

/// Type of HyprctlError
#[derive(Debug, PartialEq, Eq)]
enum HyprctlErrorKind {
    /// When try set wallpaper
    WallpaperSet,
}

impl Error for HyprctlError {}

impl fmt::Display for HyprctlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            HyprctlErrorKind::WallpaperSet => {
                write!(
                    f,
                    "run hyprctl hyprpaper wallpaper error received.\nDescription: {}",
                    &self.description
                )
            }
        }
    }
}

/// Checks the path is contained in the string
fn is_wallpaper_path(text: &str) -> bool {
    text.contains(".png")
        || text.contains(".jpg")
        || text.contains(".jpeg")
        || text.contains(".jxl")
}

/// Set new wallpaper using `hyprctl hyprpaper wallpaper`
pub fn set_new_wallpaper(new_wallpaper: Wallpaper) -> Result<(), HyprpaperPickerError> {
    let settings = format!("{},{}", new_wallpaper.monitor.0, new_wallpaper.path.0);

    let wallpaper_set = Command::new("hyprctl")
        .args(["hyprpaper", "wallpaper", settings.as_str()])
        .output()?;

    if !wallpaper_set.status.success() {
        return Err(HyprpaperPickerError::Hyprctl(HyprctlError {
            kind: HyprctlErrorKind::WallpaperSet,
            description: String::from_utf8(wallpaper_set.stdout)?,
        }));
    }

    if !String::from_utf8(wallpaper_set.stdout.clone())?.contains("ok") {
        return Err(HyprpaperPickerError::Hyprctl(HyprctlError {
            kind: HyprctlErrorKind::WallpaperSet,
            description: String::from_utf8(wallpaper_set.stdout)?,
        }));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper.png", true)]
    #[case("/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper.jpg", true)]
    #[case("/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper.jxl", true)]
    #[case("/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper.jpeg", true)]
    #[case("/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper.svg", false)]
    fn valid_wallpaper_path_in_string(#[case] text: &str, #[case] expected: bool) {
        let result = is_wallpaper_path(text);
        assert_eq!(result, expected);
    }
}
