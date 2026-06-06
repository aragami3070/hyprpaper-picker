use std::string::FromUtf8Error;

use crate::hyprctl;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HyprpaperPickerError {
    #[error("Hyprctl error: {0}")]
    Hyprctl(#[from] hyprctl::HyprctlError),

    #[error("From utf8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Toml error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Toml aboba error: {0}")]
    TomlDeser(#[from] toml::de::Error),
}
