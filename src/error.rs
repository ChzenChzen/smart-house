use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Formatter};

pub enum HouseError {
    DeviceAlreadyExistsError(String),
    RoomNotFoundError(String),
    DeviceNotFoundError(String),
    RoomAlreadyExistsError(String),
}

impl fmt::Display for HouseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use HouseError::*;
        match self {
            RoomNotFoundError(e) => write!(f, "Room `{e}` not found"),
            RoomAlreadyExistsError(e) => write!(f, "Room `{e}` already exists"),
            DeviceAlreadyExistsError(e) => write!(f, "Device `{e}` already exists"),
            DeviceNotFoundError(e) => write!(f, "Device not found: {}", e),
        }
    }
}

impl Debug for HouseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl Error for HouseError {}

fn error_chain_fmt(error: &impl Error, f: &mut Formatter<'_>) -> fmt::Result {
    writeln!(f, "{}\n", error)?;
    let mut current = error.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }

    Ok(())
}
