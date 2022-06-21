use smart_house::*;

#[test]
fn add_device_to_room_through_house_api() {
    let mut house: House<DeviceKind> = House::new("White House".to_string());
    let red = Room::new("Red Room".to_string());
    let white = Room::new("White Room".to_string());
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
    let mut house: House<DeviceKind> = House::new("White House".to_string());
    let red = Room::new("Red Room".to_string());
    let white = Room::new("White Room".to_string());
    let socket = Socket::new("Spy socket", true).into();
    let thermo = Thermo::new("Thermo", false).into();
    house.add(red.clone()).unwrap();
    house.add(white.clone()).unwrap();

    house.add_device_to_room(socket, red.name()).unwrap();
    house.add_device_to_room(thermo, red.name()).unwrap();

    house
        .remove_device_for_room(socket.name(), red.name())
        .unwrap();

    assert_eq!(house.iter().count(), 2);
    assert_eq!(house.iter().collect::<Vec<_>>()[0].iter().count(), 1);
}
