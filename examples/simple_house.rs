use smart_house::{DeviceInfoReport, DeviceKind, House, Manager, Room, Socket, Thermo};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
    Ok(())
}
