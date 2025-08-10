use std::error::Error;

use crate::hyprctl::{ActiveWallpaper, NewWallpaper, Wallpaper};

/// Get random NewWallpaper from Wallpaper vec
pub fn random_wallpaper(
    mut wallpapers: Vec<Wallpaper>,
    active_wallpaper: ActiveWallpaper,
) -> Result<NewWallpaper, Box<dyn Error>> {
    wallpapers.retain(|x| x.path != active_wallpaper.0.path);

    let rand_num = rand::random_range(0..wallpapers.len() - 1);

    let new_wallpaper = NewWallpaper(Wallpaper {
        path: wallpapers[rand_num].path.to_owned(),
        monitor: active_wallpaper.0.monitor,
    });

    Ok(new_wallpaper)
}

/// Get next NewWallpaper from Wallpaper vec
pub fn next_wallpaper(
    wallpapers: Vec<Wallpaper>,
    active_wallpaper: ActiveWallpaper,
) -> Result<NewWallpaper, Box<dyn Error>> {
    let active_wallp_index = wallpapers
        .iter()
        .position(|w| w.path == active_wallpaper.0.path);

    let new_wallpaper = match active_wallp_index {
        Some(i) => {
            if let Some(value) = wallpapers.get(i + 1) {
                value.to_owned()
            } else {
                wallpapers[0].to_owned()
            }
        }
        None => wallpapers[0].to_owned(),
    };

    Ok(NewWallpaper(Wallpaper {
        path: new_wallpaper.path,
        monitor: active_wallpaper.0.monitor,
    }))
}

pub fn prev_wallpaper(
    wallpapers: Vec<Wallpaper>,
    active_wallpaper: ActiveWallpaper,
) -> Result<NewWallpaper, Box<dyn Error>> {
    let active_wallp_index = wallpapers
        .iter()
        .position(|w| w.path == active_wallpaper.0.path);

    let new_wallpaper = match active_wallp_index {
        Some(i) => {
            if i > 0
                && let Some(value) = wallpapers.get(i - 1)
            {
                value.to_owned()
            } else {
                wallpapers[wallpapers.len() - 1].to_owned()
            }
        }
        None => wallpapers[wallpapers.len() - 1].to_owned(),
    };

    Ok(NewWallpaper(Wallpaper {
        path: new_wallpaper.path,
        monitor: active_wallpaper.0.monitor,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hyprctl::{Monitor, Path};
    use rstest::rstest;

    #[rstest]
    #[case(
        "/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper4.png",
        "/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper5.png"
    )]
    #[case(
        "/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper7.png",
        "/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper5.png"
    )]
    fn valid_next_wallpaper_choose(#[case] active_wallp: &str, #[case] except: &str) {
        let active_wallpaper = Wallpaper {
            path: Path(active_wallp.to_owned()),
            monitor: Monitor("eDP-1".to_owned()),
        };

        let except_wallpaper = Wallpaper {
            path: Path(except.to_owned()),
            monitor: Monitor("eDP-1".to_owned()),
        };

        let mut wallpapers: Vec<Wallpaper> = Vec::new();
        wallpapers.push(active_wallpaper.clone());
        wallpapers.push(except_wallpaper.clone());
        wallpapers.push(Wallpaper {
            path: Path("/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper6.png".to_owned()),
            monitor: Monitor("".to_owned()),
        });

        wallpapers.sort_by_key(|a| a.path.0.clone());

        assert_eq!(
            next_wallpaper(wallpapers, ActiveWallpaper(active_wallpaper)).unwrap(),
            NewWallpaper(except_wallpaper)
        );
    }

    #[rstest]
    #[case(
        "/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper5.png",
        "/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper4.png"
    )]
    #[case(
        "/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper5.png",
        "/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper7.png"
    )]
    fn valid_prev_wallpaper_choose(#[case] active_wallp: &str, #[case] except: &str) {
        let active_wallpaper = Wallpaper {
            path: Path(active_wallp.to_owned()),
            monitor: Monitor("eDP-1".to_owned()),
        };

        let except_wallpaper = Wallpaper {
            path: Path(except.to_owned()),
            monitor: Monitor("eDP-1".to_owned()),
        };

        let mut wallpapers: Vec<Wallpaper> = Vec::new();
        wallpapers.push(active_wallpaper.clone());
        wallpapers.push(except_wallpaper.clone());
        wallpapers.push(Wallpaper {
            path: Path("/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper6.png".to_owned()),
            monitor: Monitor("".to_owned()),
        });

        wallpapers.sort_by_key(|a| a.path.0.clone());

        assert_eq!(
            prev_wallpaper(wallpapers, ActiveWallpaper(active_wallpaper)).unwrap(),
            NewWallpaper(except_wallpaper)
        );
    }
}
