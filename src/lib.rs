trait Manager {
    type Output;

    fn name(&self) -> &str;
    fn list(&self) -> &[Self::Output];
    fn add(&mut self, item: Self::Output);
    fn remove(&mut self, item: &str);
}

#[derive(Debug, Clone)]
struct House {
    name: String,
    rooms: Vec<Room>,
}

impl House {
    fn new(name: String) -> Self {
        Self {
            name,
            rooms: Vec::new(),
        }
    }

    fn add_to_room(&mut self, device: Device, room_name: &str) {
        self.rooms
            .iter_mut()
            .find(|r| r.name == room_name)
            .unwrap_or_else(|| panic!("Couldn't find room with name {room_name}"))
            .add(device);
    }

    fn remove_from_room(&mut self, device_name: &str, room_name: &str) {
        self.rooms
            .iter_mut()
            .find(|r| r.name == room_name)
            .unwrap_or_else(|| panic!("Couldn't find room with name {room_name}"))
            .remove(device_name);
    }

    fn list_from_room(&self, room_name: &str) -> &[Device] {
        self.rooms
            .iter()
            .find(|r| r.name == room_name)
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
        self.rooms.retain(|r| r.name != room);
    }
}

#[derive(Debug, Clone)]
struct Room {
    name: String,
    devices: Vec<Device>,
}

impl Room {
    fn new(name: String) -> Self {
        Self {
            name,
            devices: Vec::new(),
        }
    }
}

impl Manager for Room {
    type Output = Device;

    fn name(&self) -> &str {
        &self.name
    }

    fn list(&self) -> &[Device] {
        &self.devices
    }

    fn add(&mut self, device: Device) {
        self.devices.push(device);
    }

    fn remove(&mut self, device: &str) {
        self.devices.retain(|d| d.name != device);
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Device {
    name: String,
}

impl DeviceInfo for Device {
    fn name(&self) -> &str {
        self.name()
    }

    fn state(&self) -> &str {
        "on"
    }
}

impl Device {
    fn new(name: String) -> Self {
        Self { name }
    }

    fn name(&self) -> &str {
        &self.name
    }
}

trait DeviceInfo {
    fn name(&self) -> &str;
    fn state(&self) -> &str;
}

fn report(info_provider: impl DeviceInfo) -> String {
    format!(
        "device name: {}\ndevice state: {}",
        info_provider.name(),
        info_provider.state()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_device_info() {
        let device = Device::new("device".to_string());
        assert_eq!(report(device), "device name: device\ndevice state: on");
    }

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
    fn add_device_to_room_through_house_api() {
        let mut house = House::new("House".to_string());
        let red = Room::new("Red Room".to_string());
        let white = Room::new("White Room".to_string());
        let tv = Device::new("TV".to_string());
        let router = Device::new("Router".to_string());
        house.add(red.clone());
        house.add(white.clone());

        house.add_to_room(tv, red.name());
        house.add_to_room(router, red.name());

        assert_eq!(house.list().len(), 2);
        assert_eq!(house.list()[0].list().len(), 2);
    }

    #[test]
    fn remove_device_from_room_through_house_api() {
        let mut house = House::new("House".to_string());
        let red = Room::new("Red Room".to_string());
        let white = Room::new("White Room".to_string());
        let tv = Device::new("TV".to_string());
        let router = Device::new("Router".to_string());
        house.add(red.clone());
        house.add(white.clone());

        house.add_to_room(tv.clone(), red.name());
        house.add_to_room(router, red.name());

        house.remove_from_room(tv.name(), red.name());

        assert_eq!(house.list().len(), 2);
        assert_eq!(house.list()[0].list().len(), 1);
    }

    #[test]
    fn add_room_to_house() {
        let mut house = House::new("House".to_string());
        let room = Room::new("Room".to_string());
        house.add(room);
        assert_eq!(house.list().len(), 1);
        assert_eq!(house.list()[0].name(), "Room");
    }

    #[test]
    fn add_device_to_room() {
        let mut room = Room::new("Room".to_string());
        let device = Device::new("Device".to_string());
        room.add(device);
        assert_eq!(room.list().len(), 1);
        assert_eq!(room.list()[0].name(), "Device");
    }

    #[test]
    fn add_device_to_room_and_room_to_house() {
        let mut house = House::new("House".to_string());
        let mut room = Room::new("Room".to_string());
        let device = Device::new("Device".to_string());
        room.add(device);
        house.add(room);
        assert_eq!(house.list().len(), 1);
        assert_eq!(house.list()[0].name(), "Room");
        assert_eq!(house.list()[0].list()[0].name, "Device");
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

    #[test]
    fn remove_device_from_room() {
        let mut room = Room::new("Room".to_string());
        let device = Device::new("Device".to_string());
        room.add(device);
        assert_eq!(room.list().len(), 1);
        room.remove("Device");
        assert_eq!(room.list().len(), 0);
    }
}
