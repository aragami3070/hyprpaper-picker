use std::{error::Error, fmt, process::Command};

pub struct Wallpaper {
    pub path: String,
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
            ErrorVariant::ListActive(err) => Some(err),
        }
    }
}

impl fmt::Display for HyprctlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.variant {
            ErrorVariant::ListActive(err) => {
                write!(
                    f,
                    "run hyprctl hyprpaper listactive error received.\nDescription: {}",
                    err.description
                )
            }
        }
    }
}

pub fn get_active_wallpaper() -> Result<ActiveWallpaper, Box<dyn Error>> {
    let list_active = Command::new("hyprctl")
        .args(&["hyprpaper", "listactive"])
        .output()?;

    if !list_active.status.success() {
        return Err(Box::new(HyprctlError {
            variant: ErrorVariant::ListActive(HyprpaperError {
                description: String::from_utf8(list_active.stderr)?,
            }),
        }));
    }

    let active_wallpaper = String::from_utf8(list_active.stdout)?;

    Ok(ActiveWallpaper(Wallpaper {
        path: active_wallpaper,
    }))
}
