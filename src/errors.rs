use std::string::FromUtf8Error;

use crate::hyprctl;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HyprpaperPickerError {
    #[error("Hyprctl error: {0}")]
    Hyprctl(#[from] hyprctl::HyprctlError),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Input error: {0}")]
    Input(String),

    #[error("From utf8 error: {0}")]
    FromUtf8(#[from] FromUtf8Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Toml serialize error: {0}")]
    TomlSer(#[from] toml::ser::Error),

    #[error("Toml deserialize error: {0}")]
    TomlDeser(#[from] toml::de::Error),
}
