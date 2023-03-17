use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use home::devices::socket::{Socket, SocketRequestHandler};
use home::network::tcp_entity::TcpCommunication;
use home::place::home::{Home, HomeRequestHandler};

fn main() {
    println!("smart_home. create");
    let mut arc_home = Arc::new(Mutex::new(Home::new("MyHome")));
    let mut home_request_handler = HomeRequestHandler::new(arc_home.clone(), arc_home.lock().unwrap().commands.clone(), arc_home.lock().unwrap().tcp_clients.clone());

    let home_start_receive = arc_home.clone();


    println!("smart_home. start receives");
    home_start_receive.lock().unwrap().start_receive();
    home_request_handler.job_analyze_commands();
    home_request_handler.job_send_commands_out();

    thread::sleep(Duration::from_secs(1));

    println!("socket. create");
    let mut socket = Arc::new(Mutex::new(Socket::from("S01", "S01 Description", 1000.0)));
    let socket_receive = socket.clone();

    let mut socket_request_handler = SocketRequestHandler::new(socket.clone(), socket.lock().unwrap().commands.clone(), socket.lock().unwrap().tcp_clients.clone());
    socket_request_handler.job_analyze_commands();
    socket_request_handler.job_send_commands_out();

    println!("socket. start receive");
    socket_receive.lock().unwrap().start_receive();

    thread::sleep(Duration::from_secs(1));
    println!("socket. try register");
    socket_receive.lock().unwrap().try_register().unwrap();

    let mut rounds = 0;

    loop {
        println!("round {}", rounds);
        thread::sleep(Duration::from_secs(1));
        rounds += 1;
    }
}