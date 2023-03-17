use std::io;
use std::io::{stdout, Stdout, Write, Result, Error, ErrorKind};
use crossterm::{execute, queue, event, terminal, cursor, style::{self, Stylize}};

pub struct Screen {}

impl Screen {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&self) {
        println!("Welcome to Smart Home!");
    }

    pub fn wait_command(&self) -> Result<String> {
        Ok(self.input("Enter command >"))
    }

    pub fn alert(&mut self, message: &str) {
        println!(r#"
        !!! Alert !!!
        {}
        !!!
        "#, message)
    }

    pub fn input(&self, label: &str) -> String {
        let mut buf = String::new();

        println!("{}", label);
        io::stdin().read_line(&mut buf).expect("Failed to read line from input");
        buf
    }

    fn read_char(&self) -> Result<char> {
        loop {
            if let Ok(event::Event::Key(event::KeyEvent {
                                            code: event::KeyCode::Char(c),
                                            ..
                                        })) = event::read()
            {
                return Ok(c);
            }
        }
    }

    pub fn draw_screen(&self, text: &str) {
        println!("\n{}\n\n", text)
    }

    pub fn draw_menu(&self, menu: &str) {
        println!("\n{}\n", menu);
    }
}