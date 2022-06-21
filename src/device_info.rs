use crate::device::DeviceKind;
pub trait DeviceInfo {
    fn name(&self) -> &str;
    fn state(&self) -> bool;
}

macro_rules! impl_device_info {
    ($($device:ident),* $(,)?) => {
        $(
            impl DeviceInfo for $device {
                fn name(&self) -> &str {
                    &self.name
                }

                fn state(&self) -> bool {
                    self.state
                }
            }
        )*
    };
}

use crate::{House, HouseError};
pub(crate) use impl_device_info;

impl DeviceInfo for DeviceKind {
    fn name(&self) -> &str {
        match self {
            DeviceKind::Socket(s) => s.name(),
            DeviceKind::Thermo(t) => t.name(),
        }
    }

    fn state(&self) -> bool {
        match self {
            DeviceKind::Socket(s) => s.state(),
            DeviceKind::Thermo(t) => t.state(),
        }
    }
}

pub trait DeviceInfoReportComposer<T> {
    fn compose(&self, house: &House<T>) -> Result<String, HouseError>;
}

pub struct DeviceInfoReport {
    pub device_name: String,
    pub room_name: String,
}
impl DeviceInfoReport {
    pub fn new(device_name: impl AsRef<str>, room_name: impl AsRef<str>) -> Self {
        Self {
            device_name: device_name.as_ref().to_owned(),
            room_name: room_name.as_ref().to_owned(),
        }
    }
}

impl<T> DeviceInfoReportComposer<T> for DeviceInfoReport
where
    T: DeviceInfo,
{
    fn compose(&self, house: &House<T>) -> Result<String, HouseError> {
        let mut devices_iterator = house
            .iter_devices_for_room(self.room_name.as_str())
            .ok_or_else(|| HouseError::RoomNotFoundError(self.room_name.to_owned()))?;

        match devices_iterator.find(|d| d.name() == self.device_name) {
            None => Err(HouseError::DeviceNotFoundError(self.device_name.to_owned())),
            Some(device) => Ok(format!(
                "Device with name {} and state {} was found in room {}",
                device.name(),
                device.state(),
                self.room_name,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Manager, Room, Socket, Thermo};

    #[test]
    fn report_produces_correct_string() {
        let thermo = Thermo::new("Thermo", true).into();
        let socket = Socket::new("Socket", true).into();
        let mut room: Room<DeviceKind> = Room::new("Kitchen");
        room.add(thermo).unwrap();
        room.add(socket).unwrap();

        let mut house = House::new("Bunker");
        house.add(room).unwrap();

        let report = DeviceInfoReport::new("Thermo", "Kitchen");
        let report = house.report(report);
        assert_eq!(
            report,
            "Device with name Thermo and state true was found in room Kitchen"
        );
    }
}
