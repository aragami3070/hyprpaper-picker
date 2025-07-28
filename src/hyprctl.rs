use std::{error::Error, fmt, process::Command};

#[derive(Debug, PartialEq, Eq)]
pub struct Wallpaper {
    pub path: String,
    pub monitor: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActiveWallpaper(pub Wallpaper);

#[derive(Debug, PartialEq, Eq)]
enum HyprctlErrorKind {
    ListActive,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HyprctlError {
    kind: HyprctlErrorKind,
    description: String,
}

impl Error for HyprctlError {}

impl fmt::Display for HyprctlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            HyprctlErrorKind::ListActive => {
                write!(
                    f,
                    "run hyprctl hyprpaper listactive error received.\nDescription: {}",
                    &self.description
                )
            }
        }
    }
}

fn is_wallpaper_path_in_string(text: String) -> Result<ActiveWallpaper, Box<dyn Error>> {
    if !(text.contains(".png")
        || text.contains(".jpg")
        || text.contains(".jpeg")
        || text.contains(".jxl"))
    {
        return Err(Box::new(HyprctlError {
            kind: HyprctlErrorKind::ListActive,
            description: text,
        }));
    }

    Ok(ActiveWallpaper(Wallpaper {
        path: text.clone(),
        monitor: text,
    }))
}

pub fn get_active_wallpaper() -> Result<ActiveWallpaper, Box<dyn Error>> {
    let list_active = Command::new("hyprctl")
        .args(["hyprpaper", "listactive"])
        .output()?;

    if !list_active.status.success() {
        return Err(Box::new(HyprctlError {
            kind: HyprctlErrorKind::ListActive,
            description: String::from_utf8(list_active.stderr)?,
        }));
    }

    let active_wallpaper = is_wallpaper_path_in_string(String::from_utf8(list_active.stdout)?)?;

    Ok(active_wallpaper)
}
