use crate::device::Device;
use crate::manager::Manager;

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
        self.devices.retain(|d| d.name() != device);
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    name: String,
    devices: Vec<Device>,
}

impl Room {
    pub fn new(name: String) -> Self {
        Self {
            name,
            devices: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_device_to_room() {
        let mut room = Room::new("Room".to_string());
        let device = Device::new("Device".to_string());
        room.add(device);
        assert_eq!(room.list().len(), 1);
        assert_eq!(room.list()[0].name(), "Device");
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

    #[test]
    fn list_devices_from_room() {
        let mut room = Room::new("Room".to_string());
        let device = Device::new("Device".to_string());
        room.add(device);
        assert_eq!(room.list().len(), 1);
        assert_eq!(room.list()[0].name(), "Device");
    }
}
