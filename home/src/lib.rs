pub mod devices;
pub mod dispatcher;
pub mod places;
pub mod services;
pub mod stores;

#[cfg(test)]
mod tests {
    use crate::{
        devices::socket::Socket,
        places::{Home, Room},
    };
    use crate::devices::thermometer::Thermometer;

    #[test]
    fn add_rooms_work() {
        let mut home = Home::new("My little Home".to_string());

        let r_hallway = Room::new("Hallway".to_string());
        let r_living_room = Room::new("Living Room".to_string());
        let r_kitchen = Room::new("Kitchen".to_string());

        home.add_room(r_hallway);
        home.add_room(r_living_room);
        home.add_room(r_kitchen);
    }

    #[test]
    fn add_device_work() {
        let mut home = Home::new("home".to_string());
        let room_name_01 = "R01";
        let room_01 = Room::new(room_name_01.to_string());
        home.add_room(room_01);

        let device_01 = Socket::new();
        home.add_device(room_name_01, Box::new(device_01)).unwrap();

        home.print_report();
    }

    #[test]
    fn connect_device_work() {
        let mut home = Home::new("home".to_string());

        let room_name_01 = "R01";
        let room01 = Room::new(room_name_01.to_string());
        home.add_room(room01);

        let s01_name = "S01";
        let s01 = Socket::from(s01_name, "Description S01", 1500.0);

        let t01_name = "T01";
        let t01 = Thermometer::from(t01_name, "Description of T01");

        home.add_device(room_name_01, Box::new(t01)).unwrap();
        home.add_device(room_name_01, Box::new(s01)).unwrap();

        home.connect_device(room_name_01, s01_name, t01_name).unwrap();

        home.print_report();
    }

    #[test]
    fn report_home_work() {
        let mut home = Home::new("home".to_string());

        let room_name_01 = "R01";
        let room01 = Room::new(room_name_01.to_string());
        home.add_room(room01);

        let room_name_02 = "R02";
        let room02 = Room::new(room_name_02.to_string());
        home.add_room(room02);

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
        let mut home = Home::new("home".to_string());

        let room_name_01 = "R01";
        let room01 = Room::new(room_name_01.to_string());
        home.add_room(room01);

        let s01_name = "S01";
        let s01 = Socket::from(s01_name, "Description S01", 1500.0);

        let t01_name = "T01";
        let t01 = Thermometer::from(t01_name, "Description of T01");

        home.add_device(room_name_01, Box::new(s01)).unwrap();
        home.add_device(room_name_01, Box::new(t01)).unwrap();

        home.connect_device(room_name_01, s01_name, t01_name).unwrap();

        home.print_schema();
    }

    #[test]
    fn found_device_work() {
        let mut home = Home::new("home".to_string());

        let room_name_01 = "R01";
        let room01 = Room::new(room_name_01.to_string());
        home.add_room(room01);

        let s01_name = "S01";
        let s01 = Socket::from(s01_name, "Description S01", 1500.0);

        let t01_name = "T01";
        let t01 = Thermometer::from(t01_name, "Description of T01");

        home.add_device(room_name_01, Box::new(t01)).unwrap();
        home.add_device(room_name_01, Box::new(s01)).unwrap();

        home.connect_device(room_name_01, s01_name, t01_name).unwrap();

        match home.device("S02") {
            Some(_) => panic!("The device not contains in the home"),
            None => println!("test work!")
        }

        match home.device("S01") {
            Some(_) => println!("test work!"),
            None => panic!("The device contains in the home")
        }
    }
}
