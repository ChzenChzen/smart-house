use crate::{DeviceInfo, DeviceInfoReportComposer, HouseError, Manager, Room};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct House<T> {
    pub name: String,
    pub rooms: HashMap<String, Room<T>>,
}

impl<T: DeviceInfo> House<T> {
    pub fn new<V: AsRef<str>>(name: V) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            rooms: HashMap::new(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Room<T>> {
        self.rooms.values()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Thermo;

    #[test]
    fn list_devices_from_room() {
        let mut club = House::new("Club");
        let mut dance_hall = Room::new("Dance-hall");
        let thermo = Thermo::new("thermo", false);
        dance_hall.add(thermo.clone()).unwrap();
        club.add(dance_hall).unwrap();
        let actual: Vec<_> = club.iter_devices_for_room("Dance-hall").unwrap().collect();

        assert_eq!(&actual, &[&thermo]);
    }

    #[test]
    fn add_room_to_house_and_check_name() {
        let mut club = House::new("Club");
        let mut dance_hall = Room::new("Dance-hall");
        let thermo = Thermo::new("thermo", false);
        dance_hall.add(thermo).unwrap();
        club.add(dance_hall.clone()).unwrap();
        assert_eq!(club.iter().nth(0).unwrap().name(), dance_hall.name());
    }

    #[test]
    fn remove_room_from_house() {
        let mut club = House::new("Club");
        let mut dance_hall = Room::new("Dance-hall");
        let thermo = Thermo::new("thermo", false);
        dance_hall.add(thermo).unwrap();
        club.add(dance_hall).unwrap();
        club.remove("Dance-hall").unwrap();
        assert_eq!(club.iter().count(), 0);
    }
}
