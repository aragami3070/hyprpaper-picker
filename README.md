# Hyprpaper picker

Blazing fast wallpaper changer for Hyprland.

<details>
	<summary>Table of Contents</summary>

* [Requirements](#Requirements)
* [Installation (only manual for now)](#Installation)
* [Usage](#Usage)
* [Arguments](#Arguments)
* [Flags](#Flags)
* [Contributing](#Contributing)
* [License](#License)
</details>


## Requirements

- [Rust with Cargo](https://www.rust-lang.org/tools/install)
- [Hyprland](https://github.com/hyprwm/Hyprland)
- [Hyprpaper](https://github.com/hyprwm/Hyprpaper)

## Installation

* Clone the repository
	```bash
	git clone https://github.com/aragami3070/hyprpaper-picker.git
	cd hyprpaper-picker
	```
* Build the project
	```bash
	cargo build --release
	```
* Add the binary to your /bin directory (this important because from other directories hyprland can't find it)
	```bash
	ln --symbolic target/release/hyprpaper-picker /bin/hyprpaper-picker
	```

## Usage
You can bind the command to a key in Hyprland.

For example, if you can bind the command to `Super+Shift+W` key, to change on random wallpaper from dir:
```bash
# In hyprland.conf
$mainMod = SUPER # Sets "Windows" key as main modifier
# Path to the directory with wallpapers
$wallpaperDir = $(dirname $(hyprctl hyprpaper listactive | head -n1 | awk -F'=' '{print $2}' | awk -F'.' '{print $1"."$2}'))
# Or you can hardcode it like this
# $wallpaperDir = /home/aragami3070/wallpapers
bind = $mainMod SHIFT, W, exec, hyprpaper-picker rand -d $wallpaperDir
```

### Arguments
- `rand` - choose random wallpaper from dir
- `next` - choose next wallpaper from dir

### Flags
- `-d` - path to the directory with wallpapers

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

[License](License)
