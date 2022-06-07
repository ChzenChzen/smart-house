pub trait DeviceInfo {
    fn name(&self) -> &str;
    fn state(&self) -> &str;
}

pub fn report(info_provider: impl DeviceInfo) -> String {
    format!(
        "device name: {}\ndevice state: {}",
        info_provider.name(),
        info_provider.state()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Device;

    #[test]
    fn report_produces_correct_string() {
        let device = Device::new("smart tv".to_string());
        let report = report(device);
        assert_eq!(report, "device name: smart tv\ndevice state: on");
    }
}
