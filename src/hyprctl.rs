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

/// Get first path and monitor name from string
///
/// If '=' not one in text then remove paths after first path
fn string_to_path_and_monitor(text: String) -> Result<(String, String), Box<dyn Error>> {
    let mut eq_pos = match text.find('=') {
        Some(pos) => pos,
        None => {
            return Err(Box::new(HyprctlError {
                kind: HyprctlErrorKind::ListActive,
                description: text,
            }));
        }
    };

    let monitor = text[..eq_pos - 1].to_string();
    let mut path = text[eq_pos + 2..].to_string();
    eq_pos = match path.find('=') {
        Some(pos) => pos,
        None => return Ok((path, monitor)),
    };

    path = path[..eq_pos - 2].to_string();
    return Ok((path, monitor));
}

/// Checks the path is contained in the string
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

    let (path, monitor) = string_to_path_and_monitor(text)?;

    Ok(ActiveWallpaper(Wallpaper {
        path: path,
        monitor: monitor,
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("invalid command", HyprctlError {
            kind: HyprctlErrorKind::ListActive,
            description: "invalid command".to_string(),
        })]
    #[case("some other error", HyprctlError {
            kind: HyprctlErrorKind::ListActive,
            description: "some other error".to_string(),
        })]
    fn invalid_wallpaper_path_in_string(#[case] text: &str, #[case] expected: HyprctlError) {
        let err = *is_wallpaper_path_in_string(text.to_string())
            .err()
            .unwrap()
            .downcast::<HyprctlError>()
            .unwrap();
        assert_eq!(err, expected);
    }
}
