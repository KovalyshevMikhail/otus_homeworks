use home::devices::socket::Socket;
use home::devices::thermometer::Thermometer;
use home::places::{Home, Room};

fn main() {
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

    println!("\nHome contain rooms:");
    for (index, room) in home.rooms().iter().enumerate() {
        println!("{}. {}", index, room);
    }

    println!("\nRoom {} contains devices:", room_name_01);
    for (index, device) in home.devices(room_name_01).iter().enumerate() {
        println!("{}. {}", index, device);
    }

    println!("\nFull list of devices with stats.");
    home.print_report();

    println!("\nSchema of the Home.");
    home.print_schema();
}
