use smart_house::*;

#[test]
fn add_device_to_room_through_house_api() {
    let mut house: House<DeviceKind> = House::new("White House");
    let red = Room::new("Red Room");
    let white = Room::new("White Room");
    let socket = Socket::new("Spy socket", true).into();
    let thermo = Thermo::new("Thermo", false).into();
    house.add(red.clone()).unwrap();
    house.add(white.clone()).unwrap();

    house.add_device_to_room(socket, red.name()).unwrap();
    house.add_device_to_room(thermo, red.name()).unwrap();

    assert_eq!(house.iter().count(), 2);
}

#[test]
fn remove_device_from_room_through_house_api() {
    let mut house = House::new("White House");
    let red = Room::new("Red Room");
    let white = Room::new("White Room");
    let socket: DeviceKind = Socket::new("Spy socket", true).into();
    let thermo = Thermo::new("Thermo", false).into();
    house.add(red.clone()).unwrap();
    house.add(white).unwrap();

    house
        .add_device_to_room(socket.clone(), red.name())
        .unwrap();
    house.add_device_to_room(thermo, red.name()).unwrap();

    house
        .remove_device_for_room(socket.name(), red.name())
        .unwrap();

    assert_eq!(house.iter().count(), 2);
    let devices_in_red_room = house.iter_devices_for_room(red.name()).unwrap().count();
    assert_eq!(devices_in_red_room, 1);
}
