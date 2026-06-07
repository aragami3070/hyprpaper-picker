use crate::hyprctl::Wallpaper;

/// Get random Wallpaper from wallpapers
pub fn random_wallpaper(mut wallpapers: Vec<Wallpaper>, active_wallpaper: Wallpaper) -> Wallpaper {
    match wallpapers.len() {
        0 => active_wallpaper,
        1 => wallpapers[0].clone(),
        _ => {
            wallpapers.retain(|x| x.path != active_wallpaper.path);

            let rand_num = rand::random_range(0..wallpapers.len() - 1);

            Wallpaper {
                path: wallpapers[rand_num].path.clone(),
                monitor: active_wallpaper.monitor,
            }
        }
    }
}

/// Get next Wallpaper from wallpapers
pub fn next_wallpaper(wallpapers: &[Wallpaper], active_wallpaper: Wallpaper) -> Wallpaper {
    let active_wallp_index = wallpapers
        .iter()
        .position(|w| w.path == active_wallpaper.path);

    let new_wallpaper = match active_wallp_index {
        Some(i) => wallpapers[(i + 1) % wallpapers.len()].clone(),
        None => wallpapers[0].clone(),
    };

    Wallpaper {
        path: new_wallpaper.path,
        monitor: active_wallpaper.monitor,
    }
}

/// Get prev Wallpaper from wallpapers
pub fn prev_wallpaper(wallpapers: &[Wallpaper], active_wallpaper: Wallpaper) -> Wallpaper {
    let active_wallp_index = wallpapers
        .iter()
        .position(|w| w.path == active_wallpaper.path);

    let new_wallpaper = match active_wallp_index {
        Some(i) => wallpapers[(i + wallpapers.len() - 1) % wallpapers.len()].clone(),
        None => wallpapers[wallpapers.len() - 1].clone(),
    };

    Wallpaper {
        path: new_wallpaper.path,
        monitor: active_wallpaper.monitor,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hyprctl::Monitor;
    use rstest::rstest;
    use std::path::PathBuf;

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
            path: PathBuf::from(active_wallp),
            monitor: Monitor("eDP-1".to_string()),
        };

        let except_wallpaper = Wallpaper {
            path: PathBuf::from(except),
            monitor: Monitor("eDP-1".to_string()),
        };

        let mut wallpapers: Vec<Wallpaper> = vec![
            active_wallpaper.clone(),
            except_wallpaper.clone(),
            Wallpaper {
                path: PathBuf::from(
                    "/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper6.png",
                ),
                monitor: Monitor("".to_string()),
            },
        ];

        wallpapers.sort_by(|a, b| a.path.cmp(&b.path));

        assert_eq!(
            next_wallpaper(&wallpapers, active_wallpaper),
            except_wallpaper
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
            path: PathBuf::from(active_wallp),
            monitor: Monitor("eDP-1".to_string()),
        };

        let except_wallpaper = Wallpaper {
            path: PathBuf::from(except),
            monitor: Monitor("eDP-1".to_string()),
        };

        let mut wallpapers: Vec<Wallpaper> = vec![
            active_wallpaper.clone(),
            except_wallpaper.clone(),
            Wallpaper {
                path: PathBuf::from(
                    "/home/aragami3070/.config/hypr/Wallpapers/Other/wallpaper6.png",
                ),
                monitor: Monitor("".to_string()),
            },
        ];

        wallpapers.sort_by(|a, b| a.path.cmp(&b.path));

        assert_eq!(
            prev_wallpaper(&wallpapers, active_wallpaper),
            except_wallpaper
        );
    }
}
