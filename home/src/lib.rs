pub mod devices;
pub mod dispatcher;
pub mod places;
pub mod services;
pub mod stores;
pub mod network;

#[cfg(test)]
mod tests {
    use crate::devices::thermometer::Thermometer;
    use crate::{
        devices::socket::Socket,
        places::{Home, Room},
    };

    #[test]
    fn add_rooms_work() {
        let mut home = Home::new("My little Home");

        let r_hallway = Room::new("Hallway");
        let r_living_room = Room::new("Living Room");
        let r_kitchen = Room::new("Kitchen");

        home.add_room(r_hallway).unwrap();
        home.add_room(r_living_room).unwrap();
        home.add_room(r_kitchen).unwrap();
    }

    #[test]
    fn add_device_work() {
        let mut home = Home::new("home");
        let room_name_01 = "R01";
        let room_01 = Room::new(room_name_01);
        home.add_room(room_01).unwrap();

        let device_01 = Socket::new();
        home.add_device(room_name_01, Box::new(device_01)).unwrap();

        home.print_report();
    }

    #[test]
    fn connect_device_work() {
        let mut home = Home::new("home");

        let room_name_01 = "R01";
        let room01 = Room::new(room_name_01);
        home.add_room(room01).unwrap();

        let s01_name = "S01";
        let s01 = Socket::from(s01_name, "Description S01", 1500.0);

        let t01_name = "T01";
        let t01 = Thermometer::from(t01_name, "Description of T01");

        home.add_device(room_name_01, Box::new(t01)).unwrap();
        home.add_device(room_name_01, Box::new(s01)).unwrap();

        home.connect_device(room_name_01, s01_name, t01_name)
            .unwrap();

        home.print_report();
    }

    #[test]
    fn report_home_work() {
        let mut home = Home::new("home");

        let room_name_01 = "R01";
        let room01 = Room::new(room_name_01);
        home.add_room(room01).unwrap();

        let room_name_02 = "R02";
        let room02 = Room::new(room_name_02);
        home.add_room(room02).unwrap();

        let s01 = Socket::from("S01", "Description S01", 1500.0);
        let s02 = Socket::from("S02", "Description S02", 1200.0);

        let t01 = Thermometer::from("T01", "Description of T01");

        home.add_device(room_name_01, Box::new(t01)).unwrap();

        home.add_device(room_name_02, Box::new(s01)).unwrap();
        home.add_device(room_name_02, Box::new(s02)).unwrap();

        home.print_report();
    }

    #[test]
    fn schema_home_work() {
        let mut home = Home::new("home");

        let room_name_01 = "R01";
        let room01 = Room::new(room_name_01);
        home.add_room(room01).unwrap();

        let s01_name = "S01";
        let s01 = Socket::from(s01_name, "Description S01", 1500.0);

        let t01_name = "T01";
        let t01 = Thermometer::from(t01_name, "Description of T01");

        home.add_device(room_name_01, Box::new(s01)).unwrap();
        home.add_device(room_name_01, Box::new(t01)).unwrap();

        home.connect_device(room_name_01, s01_name, t01_name)
            .unwrap();

        home.print_schema();
    }

    #[test]
    fn found_device_work() {
        let mut home = Home::new("home");

        let room_name_01 = "R01";
        let room01 = Room::new(room_name_01);
        home.add_room(room01).unwrap();

        let s01_name = "S01";
        let s01 = Socket::from(s01_name, "Description S01", 1500.0);

        let t01_name = "T01";
        let t01 = Thermometer::from(t01_name, "Description of T01");

        home.add_device(room_name_01, Box::new(t01)).unwrap();
        home.add_device(room_name_01, Box::new(s01)).unwrap();

        home.connect_device(room_name_01, s01_name, t01_name)
            .unwrap();

        match home.device("S02") {
            Some(_) => panic!("The device not contains in the home"),
            None => println!("test work!"),
        }

        match home.device("S01") {
            Some(_) => println!("test work!"),
            None => panic!("The device contains in the home"),
        }
    }

    #[test]
    fn remove_room_work() {
        let mut home = Home::new("home");

        home.add_room(Room::new("R01")).unwrap();
        home.add_room(Room::new("R02")).unwrap();
        home.add_room(Room::new("R03")).unwrap();
        let rooms_before = home.rooms();

        home.remove_room("R02").unwrap();
        let rooms_after = home.rooms();

        assert_ne!(rooms_before.len(), rooms_after.len());
        assert!(rooms_before.contains(&String::from("R02")));
        assert!(!rooms_after.contains(&String::from("R02")));
    }

    #[test]
    fn remove_device_work() {
        let mut home = Home::new("home");

        let room_name = "R01";
        home.add_room(Room::new(room_name)).unwrap();
        home.add_device(room_name, Box::new(Socket::from("S01", "S01 description", 1000.0))).unwrap();
        home.add_device(room_name, Box::new(Socket::from("S02", "S02 description", 1000.0))).unwrap();
        home.add_device(room_name, Box::new(Socket::from("S03", "S03 description", 1000.0))).unwrap();
        home.add_device(room_name, Box::new(Socket::from("S04", "S04 description", 1000.0))).unwrap();

        let remove_device_name = "S03";
        let result = home.remove_device(remove_device_name);
        let devices = home.devices(room_name);

        assert!(result.is_ok());
        assert_eq!(devices.len(), 3);
        assert!(!devices.contains(&String::from(remove_device_name)))
    }
}
