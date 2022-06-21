use crate::{DeviceInfo, DeviceInfoReportComposer, HouseError, Manager, Room};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct House<T> {
    name: String,
    rooms: HashMap<String, Room<T>>,
}

impl<T: DeviceInfo> House<T> {
    pub fn new<V: AsRef<str>>(name: V) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            rooms: HashMap::new(),
        }
    }

    pub fn add_device_to_room<V: AsRef<str>>(
        &mut self,
        device: T,
        room_name: V,
    ) -> Result<(), HouseError> {
        self.rooms
            .get_mut(room_name.as_ref())
            .ok_or_else(|| HouseError::RoomNotFoundError(room_name.as_ref().to_owned()))?
            .add(device)
    }

    pub fn remove_device_for_room<V: AsRef<str>>(
        &mut self,
        device_name: V,
        room_name: V,
    ) -> Result<(), HouseError> {
        self.rooms
            .get_mut(room_name.as_ref())
            .ok_or_else(|| HouseError::RoomNotFoundError(room_name.as_ref().to_owned()))?
            .remove(device_name.as_ref())
    }

    pub fn iter_devices_for_room<V: AsRef<str>>(
        &self,
        room_name: V,
    ) -> Option<impl Iterator<Item = &T>> {
        self.rooms.get(room_name.as_ref()).map(|room| room.iter())
    }

    pub fn report<I: DeviceInfoReportComposer<T>>(&self, info_provider: I) -> String {
        info_provider
            .compose(self)
            .unwrap_or_else(|e| format!("{:?}", e))
    }
}

impl<T: DeviceInfo> Manager for House<T> {
    type Item = Room<T>;
    
    fn name(&self) -> &str {
        &self.name
    }

    fn add(&mut self, room: Self::Item) -> Result<(), HouseError> {
        match self.rooms.get(room.name()) {
            Some(_) => Err(HouseError::RoomAlreadyExistsError(room.name().to_owned())),
            None => {
                self.rooms.insert(room.name().to_owned(), room);
                Ok(())
            }
        }
    }

    fn remove(&mut self, room_name: impl AsRef<str>) -> Result<(), HouseError> {
        self.rooms.remove(room_name.as_ref()).map_or_else(
            || Err(HouseError::RoomNotFoundError(room_name.as_ref().to_owned())),
            |_| Ok(()),
        )
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::Device;
//
//     #[test]
//     fn list_devices_from_room() {
//         let mut house = House::new("House".to_string());
//         let mut room = Room::new("Room".to_string());
//         let device = Device::new("Device".to_string());
//
//         room.add(device.clone());
//         house.add(room.clone());
//
//         assert_eq!(house.list_from_room("Room"), &[device]);
//     }
//
//     #[test]
//     fn add_room_to_house_and_check_name() {
//         let mut house = House::new("House".to_string());
//         let room = Room::new("Room".to_string());
//         house.add(room);
//         assert_eq!(house.list().len(), 1);
//         assert_eq!(house.list()[0].name(), "Room");
//     }
//
//     #[test]
//     fn remove_room_from_house() {
//         let mut house = House::new("House".to_string());
//         let room = Room::new("Room".to_string());
//         house.add(room);
//         assert_eq!(house.list().len(), 1);
//         house.remove("Room");
//         assert_eq!(house.list().len(), 0);
//     }
// }
