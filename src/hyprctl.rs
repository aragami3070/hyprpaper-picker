use std::{error::Error, fmt, process::Command};

pub struct Wallpaper {
    pub path: String,
}

pub struct ActiveWallpaper(pub Wallpaper);

#[derive(Debug)]
enum HyprctlErrorKind {
    ListActive,
}

#[derive(Debug)]
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

    let active_wallpaper = String::from_utf8(list_active.stdout)?;

    Ok(ActiveWallpaper(Wallpaper {
        path: active_wallpaper,
    }))
}
