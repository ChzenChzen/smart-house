use smart_house::*;

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
fn add_device_to_room_and_room_to_house() {
    let mut house = House::new("House".to_string());
    let mut room = Room::new("Room".to_string());
    let device = Device::new("Device".to_string());
    room.add(device);
    house.add(room);
    assert_eq!(house.list().len(), 1);
    assert_eq!(house.list()[0].name(), "Room");
    assert_eq!(house.list()[0].list()[0].name(), "Device");
}
