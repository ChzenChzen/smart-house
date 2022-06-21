use crate::manager::Manager;
use crate::{DeviceInfo, HouseError};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room<T> {
    name: String,
    devices: HashMap<String, T>,
}

impl<T: DeviceInfo> Manager for Room<T> {
    type Item = T;

    fn name(&self) -> &str {
        &self.name
    }

    fn add(&mut self, device: Self::Item) -> Result<(), HouseError> {
        match self.devices.get(device.name()) {
            Some(_) => Err(HouseError::DeviceAlreadyExistsError(
                device.name().to_owned(),
            )),
            None => {
                self.devices.insert(device.name().to_owned(), device);
                Ok(())
            }
        }
    }

    fn remove(&mut self, device_name: impl AsRef<str>) -> Result<(), HouseError> {
        self.devices.remove(device_name.as_ref()).map_or_else(
            || {
                Err(HouseError::DeviceNotFoundError(
                    device_name.as_ref().to_owned(),
                ))
            },
            |_| Ok(()),
        )
    }
}

impl<T> Room<T> {
    pub fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            devices: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.devices.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DeviceKind, Socket, Thermo};

    #[test]
    fn add_device_to_room() {
        let mut room = Room::new("Kitchen");
        let device = Socket::new("Socket", false);
        assert_eq!(room.iter().count(), 0);
        room.add(device).unwrap();
        let devices: Vec<_> = room.iter().collect();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].name(), "Socket");
    }

    #[test]
    fn remove_device_from_room() {
        let mut room = Room::new("Kitchen");
        let device = Thermo::new("Thermo", false);
        room.add(device).unwrap();
        assert_eq!(room.iter().count(), 1);
        room.remove("Thermo").unwrap();
        assert_eq!(room.iter().count(), 0);
    }

    #[test]
    fn list_devices_from_room() {
        let mut room: Room<DeviceKind> = Room::new("Kitchen");
        let socket = Socket::new("Socket", false).into();
        let thermo = Thermo::new("Thermo", false).into();
        room.add(thermo).unwrap();
        room.add(socket).unwrap();
        assert_eq!(room.iter().count(), 2);
    }
}
