use crate::{Manager, Room};

#[derive(Debug, Clone)]
pub struct House {
    name: String,
    rooms: Vec<Room>,
}

impl House {
    pub fn new(name: String) -> Self {
        Self {
            name,
            rooms: Vec::new(),
        }
    }

    pub fn add_to_room(&mut self, device: Device, room_name: &str) {
        self.rooms
            .iter_mut()
            .find(|r| r.name() == room_name)
            .unwrap_or_else(|| panic!("Couldn't find room with name {room_name}"))
            .add(device);
    }

    pub fn remove_from_room(&mut self, device_name: &str, room_name: &str) {
        self.rooms
            .iter_mut()
            .find(|r| r.name() == room_name)
            .unwrap_or_else(|| panic!("Couldn't find room with name {room_name}"))
            .remove(device_name);
    }

    pub fn list_from_room(&self, room_name: &str) -> &[Device] {
        self.rooms
            .iter()
            .find(|r| r.name() == room_name)
            .unwrap_or_else(|| panic!("Couldn't find room with name {room_name}"))
            .list()
    }
}

impl Manager for House {
    type Output = Room;

    fn name(&self) -> &str {
        &self.name
    }

    fn list(&self) -> &[Room] {
        &self.rooms
    }

    fn add(&mut self, room: Room) {
        self.rooms.push(room);
    }

    fn remove(&mut self, room: &str) {
        self.rooms.retain(|r| r.name() != room);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Device;

    #[test]
    fn list_devices_from_room() {
        let mut house = House::new("House".to_string());
        let mut room = Room::new("Room".to_string());
        let device = Device::new("Device".to_string());

        room.add(device.clone());
        house.add(room.clone());

        assert_eq!(house.list_from_room("Room"), &[device]);
    }

    #[test]
    fn add_room_to_house_and_check_name() {
        let mut house = House::new("House".to_string());
        let room = Room::new("Room".to_string());
        house.add(room);
        assert_eq!(house.list().len(), 1);
        assert_eq!(house.list()[0].name(), "Room");
    }

    #[test]
    fn remove_room_from_house() {
        let mut house = House::new("House".to_string());
        let room = Room::new("Room".to_string());
        house.add(room);
        assert_eq!(house.list().len(), 1);
        house.remove("Room");
        assert_eq!(house.list().len(), 0);
    }
}
