use std::process;

mod hyprctl;
mod dir_scan;

fn main() {
    let active_wallpaper = match hyprctl::get_active_wallpaper() {
        Ok(w) => w,
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    };

    println!("Result: {active_wallpaper:?}");
}
