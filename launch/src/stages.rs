use std::io::Result;

#[derive(Debug)]
pub enum Devices {
    Socket,
    Thermometer,
}

#[derive(Debug)]
pub enum State {
    MainMenu,
    ChangeHomeTitle,
    CreateRoom,
    CreateDevice,
    Empty
}

#[derive(Debug)]
pub enum Action {
    MainMenuQuit,
    MainMenuAddDevice,
    MainMenuRemoveDevice,
    MainMenuAddRoom,
    MainMenuRemoveRoom,
    MainMenuShowSchema,
    MainMenuShowReport,

    ChangeHomeTitleQuit,
    ChangeHomeTitleChangeTitle,
    ChangeHomeTitleSave,

    CreateRoomQuit,
    CreateRoomEnterName,
    CreateRoomSave,

    CreateDeviceQuit,
    CreateDeviceSmartSocket,
    CreateDeviceSmartThermometer,

    Unsupported
}

#[derive(Debug)]
pub struct Stages {
    current_state: State,
    previous_state: State
}

impl Stages {
    pub fn new() -> Self {
        Self {
            current_state: State::MainMenu,
            previous_state: State::Empty
        }
    }

    pub fn with_state(self, new_state: State) -> Self {
        Self {
            current_state: new_state,
            previous_state: self.current_state
        }
    }

    pub fn current(&self) -> &State {
        &self.current_state
    }

    pub fn menu(&self) -> &str {
        match self.current_state {
            State::MainMenu => {
                r#"
==========================================================================================
Controls:
  'q' - exit from program
  'a' - add new device            'd' - remove device             's' - output home schema
  'r' - add new room              'c' - remove room               'w' - output home report
==========================================================================================
"#
            }
            State::ChangeHomeTitle => {
                r#"
==========================================================================================
Controls:
  'q' - return to main menu
  'c' - change title of home      's' - save title
==========================================================================================
"#
            }
            State::CreateRoom => {
                r#"
==========================================================================================
Controls:
  'q' - return to main menu
  'c' - enter room name
==========================================================================================
"#
            }
            State::CreateDevice => {
                r#"
==========================================================================================
Controls:
  'q' - return to main menu
  'ss'- add smart socket          'st'- add smart thermometer
==========================================================================================
"#
            }
            State::Empty => {r#""#}
        }
    }

    pub fn action_of(&self, key: &str) -> Action {
        match self.current_state {
            State::MainMenu => {
                match key {
                    "q" => Action::MainMenuQuit,
                    "a" => Action::MainMenuAddDevice,
                    "d" => Action::MainMenuRemoveDevice,
                    "r" => Action::MainMenuAddRoom,
                    "c" => Action::MainMenuRemoveRoom,
                    "s" => Action::MainMenuShowSchema,
                    "w" => Action::MainMenuShowReport,
                    _ => Action::Unsupported
                }
            }
            State::ChangeHomeTitle => {
                match key {
                    "q" => Action::ChangeHomeTitleQuit,
                    "c" => Action::ChangeHomeTitleChangeTitle,
                    "s" => Action::ChangeHomeTitleSave,
                    _ => Action::Unsupported
                }
            }
            State::CreateRoom => {
                match key {
                    "q" => Action::CreateRoomQuit,
                    "c" => Action::CreateRoomEnterName,
                    "s" => Action::CreateRoomSave,
                    _ => Action::Unsupported
                }
            }
            State::CreateDevice => {
                match key {
                    "q" => Action::CreateDeviceQuit,
                    "ss" => Action::CreateDeviceSmartSocket,
                    "st" => Action::CreateDeviceSmartThermometer,
                    _ => Action::Unsupported
                }
            }
            State::Empty => {
                Action::Unsupported
            }
        }
    }
}