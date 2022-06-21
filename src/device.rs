use crate::device_info::{impl_device_info, DeviceInfo};

#[derive(Debug, Clone, PartialEq)]
pub enum DeviceKind {
    Socket(Socket),
    Thermo(Thermo),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Socket {
    name: String,
    state: bool,
}

impl Socket {
    pub fn new(name: impl AsRef<str>, state: bool) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            state,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Thermo {
    name: String,
    state: bool,
}

impl Thermo {
    pub fn new(name: impl AsRef<str>, state: bool) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            state,
        }
    }
}

impl From<Socket> for DeviceKind {
    fn from(socket: Socket) -> Self {
        DeviceKind::Socket(socket)
    }
}

impl From<Thermo> for DeviceKind {
    fn from(thermo: Thermo) -> Self {
        DeviceKind::Thermo(thermo)
    }
}

impl_device_info!(Socket, Thermo);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_socket() {
        let _socket: DeviceKind = Socket::new("my socket", true).into();
    }

    #[test]
    fn test_device_thermo() {
        let _socket: DeviceKind = Thermo::new("my thermo", true).into();
    }
}
