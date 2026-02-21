use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ZBus Error")]
    ZBusError(#[from] zbus::Error),

    #[error("FDO Error")]
    FDOError(#[from] zbus::fdo::Error),

    #[error("IO Error")]
    IOError(#[from] io::Error),

    #[error("Rodio Error")]
    RodioError(#[from] rodio::PlayError),

    #[error("Couldn't compile regex")]
    RegexError(#[from] regex::Error),

    #[error("Couldn't decode string into toml object")]
    TomlDecodeError(#[from] toml::de::Error),

    #[error("Couldn't serialize struct into toml string")]
    TomlEncodeError(#[from] toml::ser::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
