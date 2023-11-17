use std::{fs, str::FromStr};
use std::io::Read;
use messages::{handle_error, MessageType};

#[derive(Debug, PartialEq)]
pub(crate) enum CmdType {
    Text,
    File,
    Image,
    Quit,
    Error,
}

#[derive(Debug)]
pub struct Command {
    pub(crate) cmd: CmdType,
    pub(crate) input: String,
}

unsafe impl Send for Command {}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseCommandError {
    pub message: &'static str,
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ".file" => Ok(Command { cmd: CmdType::File, input: "".to_string() }),
            ".image" => Ok(Command { cmd: CmdType::Image, input: "".to_string() }),
            ".quit" => Ok(Command { cmd: CmdType::Quit, input: "".to_string() }),
            _ => Ok(Command { cmd: CmdType::Text, input: s.to_string() }),
        }
    }
}

impl Command {
    pub fn process(&mut self) -> Option<MessageType> {
        match self.cmd {
            CmdType::File => {
                let buffer = self.read_file(&self.input.to_string());
                return Some(MessageType::File(self.input.to_string(), buffer));
            }
            CmdType::Image => {
                let buffer = self.read_file(&self.input.to_string());
                return Some(MessageType::Image(buffer));
            }
            _ => {
                return Some(MessageType::Text(self.input.to_string()));
            }
        }
    }

    fn read_file(&mut self, file_path: &String) -> Vec<u8> {
        let mut file = fs::File::open(file_path).map_err(|err| {
            handle_error(&format!("Error trying to open file '{file_path}'"), err);
        }).unwrap();

        let total_file_size = file.metadata().map_err(|err| {
            handle_error(&format!("Error trying to get size of file '{file_path}'"), err);
        }).unwrap().len();

        let mut buffer = vec![0; total_file_size as usize];
        file.read(&mut buffer).expect("buffer overflow");
        buffer
    }
}

pub fn match_cmd(_command: &&str, input: String) -> Command {
    let cmd = Command::from_str(_command);
    match cmd {
        Ok(mut res) => {
            res.input = input;
            res
        }
        Err(_e) => {
            Command {
                cmd: CmdType::Error,
                input: input,
            }
        }
    }
}