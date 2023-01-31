use std::io::stdout;

use crossterm::{
    cursor, event,
    execute, queue, Result, style::{self, Stylize}, terminal
};
use home::devices::socket::Socket;
use home::places::Home;

use crate::screen::Screen;
use crate::stages::{Action, Stages, State};

pub mod screen;
mod stages;

struct Program {
    home: Home,
    screen: Screen,
    stage: Stages
}

impl Program {
    pub fn new() -> Self {
        let home = Home::new("SMART_HOME");
        let screen = Screen::new();
        let stage = Stages::new();

        Self {
            home,
            screen,
            stage
        }
    }

    pub fn main_loop(&mut self) -> Result<()> {
        self.screen.init();
        let mut stage = Stages::new();
        let mut alert = Option::<&str>::None;

        loop {
            println!("---------------------------------------------------------------\n");
            match stage.current() {
                State::MainMenu => {
                    self.home.print_schema();
                }
                State::ChangeHomeTitle => {
                    println!(r#"
                        Home name before: {}
                    "#, self.home.name());
                }
                State::CreateRoom => {
                    println!("Home have rooms:");
                    for room in self.home.rooms() {
                        println!("{}", room);
                    }
                }
                State::CreateDevice => {
                    println!("Home have devices:");
                    for device in self.home.devices() {
                        println!("{}", device);
                    }
                }
                State::Empty => {}
            }


            let menu = stage.menu();
            self.screen.draw_menu(menu);

            match alert {
                None => {},
                Some(msg) => {
                    self.screen.alert(msg);
                    alert = None;
                }
            }

            let waited_command = self.screen.wait_command().unwrap();
            let action = stage.action_of(waited_command.trim());

            match action {
                Action::MainMenuQuit => {
                    break;
                }
                Action::MainMenuAddDevice => {
                    stage = stage.with_state(State::CreateDevice);
                }
                Action::MainMenuRemoveDevice => {}
                Action::MainMenuAddRoom => {
                    stage = stage.with_state(State::CreateRoom);
                }
                Action::MainMenuRemoveRoom => {}
                Action::MainMenuShowSchema => {}
                Action::MainMenuShowReport => {}
                Action::ChangeHomeTitleQuit => {}
                Action::ChangeHomeTitleChangeTitle => {}
                Action::ChangeHomeTitleSave => {}
                Action::CreateRoomQuit => {
                    stage = stage.with_state(State::MainMenu);
                }
                Action::CreateRoomEnterName => {
                    let room_name = self.screen.input("Enter room name");
                    match self.home.add_room(room_name.trim()) {
                        Ok(_) => {
                            println!("Room save!");
                        }
                        Err(err) => {
                            println!("Error in adding room!\n{}", err);
                        }
                    }
                }
                Action::CreateRoomSave => {}

                Action::CreateDeviceQuit => {
                    stage = stage.with_state(State::MainMenu);
                }
                Action::CreateDeviceSmartSocket => {
                    let name = self.screen.input("Enter device name");
                    let description = self.screen.input("Enter device description");
                    let power = self.screen.input("Enter socket power (float number)").trim().parse::<f32>().expect("Expect only float number");

                    let device = Socket::from(name.trim(), description.trim(), power);
                    self.home.add_device(self.home.rooms()[0].as_str(), Box::new(device)).unwrap();
                }
                Action::CreateDeviceSmartThermometer => {}

                Action::Unsupported => {}
            }
        }

        Ok(())
    }
}


fn main() -> Result<()> {
    let mut program = Program::new();
    program.main_loop()
}
