use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ZBus Error")]
    ZBusError(#[from] zbus::Error),

    #[error("FDO Error")]
    FDOError(#[from] zbus::fdo::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
