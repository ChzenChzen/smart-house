use crate::device_info::DeviceInfo;

#[derive(Debug, Clone, PartialEq)]
pub struct Device {
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
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use crate::device::Device;
    use crate::device_info::DeviceInfo;

    #[test]
    fn test_device_info() {
        let device = Device::new("smart tv".to_string());
        assert_eq!(device.name(), "smart tv");
        assert_eq!(device.state(), "on");
    }
}
