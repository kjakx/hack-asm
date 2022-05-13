use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(PartialEq)]
pub enum CommandType {
    A,
    C,
    L,
}

pub struct Parser {
    reader: BufReader<File>,
    current_cmd: String,
}

impl Parser {
    pub fn new(f: File) -> Self {
        Parser {
            reader: BufReader::new(f),
            current_cmd: String::new(),
        }
    }

    pub fn has_more_commands(&mut self) -> bool {
        self.reader.fill_buf().map(|b| !b.is_empty()).unwrap()
    }

    pub fn advance(&mut self) {
        if !self.has_more_commands() {
            panic!("cannot advance because no more commands");
        }
        while self.current_cmd == "" && self.has_more_commands() {
            let mut line = String::new();
            self.reader.read_line(&mut line);
            self.current_cmd = line.trim().to_string();
        }
    }

    pub fn command_type(&self) -> CommandType {
        match self.current_cmd.chars().nth(0).unwrap() {
            '@' => CommandType::A,
            '(' => CommandType::L,
            _ => CommandType::C,
        }
    }

    pub fn symbol(&self) -> String {
        match self.command_type() {
            CommandType::A => {
                let x: &[_] = &['@'];
                self.current_cmd.trim_start_matches(x).to_string()
            },
            CommandType::L => {
                let x: &[_] = &['(', ')'];
                self.current_cmd.trim_matches(x).to_string()
            },
            CommandType::C => {
                panic!("C command has no symbol");
            }
        }
    }

    pub fn dest(&self) -> String {
        if self.command_type() != CommandType::C {
            panic!("no dest in not-C command");
        }
        match self.current_cmd.split_once('=') {
            Some((dest, _)) => dest.to_string(),
            _ => String::from(""),
        }
    }

    pub fn comp(&self) -> String {
        if self.command_type() != CommandType::C {
            panic!("no comp in not-C command");
        }
        if self.current_cmd.contains("=") {
            self.current_cmd.split_once('=').unwrap().1.to_string()
        } else if self.current_cmd.contains(";") {
            self.current_cmd.split_once(';').unwrap().0.to_string()
        } else {
            String::from("")
        }
    }

    pub fn jump(&self) -> String {
        if self.command_type() != CommandType::C {
            panic!("no jump in not-C command");
        }
        match self.current_cmd.split_once(';') {
            Some((_, jump)) => jump.to_string(),
            _ => String::from(""),
        }
    }
}
