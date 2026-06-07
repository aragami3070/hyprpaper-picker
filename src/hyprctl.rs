use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, path::PathBuf, process::Command, str::FromStr};

use crate::errors::HyprpaperPickerError;

/// Monitor port (for example DP-2 or eDP-1)
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Monitor(pub String);

impl FromStr for Monitor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            Err("Monitor cannot be empty".to_string())
        } else {
            Ok(Monitor(s.to_string()))
        }
    }
}

/// Wallpaper info
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Wallpaper {
    pub path: PathBuf,
    pub monitor: Monitor,
}

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

/// Checks the path is a valid wallpaper file
pub fn is_wallpaper_path(path: &std::path::Path) -> bool {
    if let Some(ext) = path.extension() {
        ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "jxl"
    } else {
        false
    }
}

/// Set new wallpaper using `hyprctl hyprpaper wallpaper`
pub fn set_new_wallpaper(new_wallpaper: &Wallpaper) -> Result<(), HyprpaperPickerError> {
    let settings = format!(
        "{},{}",
        new_wallpaper.monitor.0,
        new_wallpaper.path.display()
    );

    let wallpaper_set = Command::new("hyprctl")
        .args(["hyprpaper", "wallpaper", settings.as_str()])
        .output()?;

    if !wallpaper_set.status.success() {
        return Err(HyprpaperPickerError::Hyprctl(HyprctlError {
            kind: HyprctlErrorKind::WallpaperSet,
            description: String::from_utf8(wallpaper_set.stderr)?,
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
        let result = is_wallpaper_path(std::path::Path::new(text));
        assert_eq!(result, expected);
    }
}
