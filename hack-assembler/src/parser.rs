use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(PartialEq)]
pub enum CommandType {
    A_COMMAND,
    C_COMMAND,
    L_COMMAND,
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

    pub fn hasMoreCommands(&mut self) -> bool {
        self.reader.fill_buf().map(|b| !b.is_empty()).unwrap()
    }

    pub fn advance(&mut self) {
        if !self.hasMoreCommands() {
            panic!("cannot advance because no more commands");
        }
        while self.current_cmd.trim() == "" {
            self.reader.read_line(&mut self.current_cmd);
        }
    }

    pub fn commandType(&self) -> CommandType {
        let cmd_trimmed = self.current_cmd.trim();
        match cmd_trimmed.chars().nth(0).unwrap() {
            '@' => CommandType::A_COMMAND,
            '(' => CommandType::L_COMMAND,
            _ => CommandType::C_COMMAND,
        }
    }

    pub fn symbol(&self) -> String {
        match self.commandType() {
            CommandType::A_COMMAND => {
                let x: &[_] = &['@'];
                self.current_cmd.trim().trim_start_matches(x).to_string()
            },
            CommandType::L_COMMAND => {
                let x: &[_] = &['(', ')'];
                self.current_cmd.trim_matches(x).to_string()
            },
            CommandType::C_COMMAND => {
                panic!("C command has no symbol");
            }
        }
    }

    pub fn dest(&self) -> String {
        if self.commandType() != CommandType::C_COMMAND {
            panic!("no dest in not-C command");
        }
        match self.current_cmd.split_once('=') {
            Some((dest, _)) => dest.to_string(),
            _ => String::from(""),
        }
    }

    pub fn comp(&self) -> String {
        if self.commandType() != CommandType::C_COMMAND {
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
        if self.commandType() != CommandType::C_COMMAND {
            panic!("no jump in not-C command");
        }
        match self.current_cmd.split_once(';') {
            Some((_, jump)) => jump.to_string(),
            _ => String::from(""),
        }
    }
}
