use std::{str::FromStr};
use slug::slugify;

#[derive(Debug, PartialEq)]
pub(crate) enum CmdType {
    Uppercase,
    Lowercase,
    Nospace,
    Slugify,
    Csv,
    Error,
    Exit,
}

#[derive(Debug)]
pub struct Command {
    pub(crate) cmd: CmdType,
    pub(crate) input: String,
    pub(crate) result: String,
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
            "uppercase" => Ok(Command { cmd: CmdType::Uppercase, input: "".to_string(), result: "".to_string() }),
            "lowercase" => Ok(Command { cmd: CmdType::Lowercase, input: "".to_string(), result: "".to_string() }),
            "nospaces" => Ok(Command { cmd: CmdType::Nospace, input: "".to_string(), result: "".to_string() }),
            "slugify" => Ok(Command { cmd: CmdType::Slugify, input: "".to_string(), result: "".to_string() }),
            "csv" => Ok(Command { cmd: CmdType::Csv, input: "".to_string(), result: "".to_string() }),
            "exit" => Ok(Command { cmd: CmdType::Exit, input: "".to_string(), result: "".to_string() }),
            _ => Err(ParseCommandError { message: "Unknown command!" }),
        }
    }
}

impl Command {
    pub fn result(&self) -> String {
        return self.result.clone();
    }

    pub fn process(&mut self) {
        self.result = match self.cmd {
            CmdType::Uppercase => {
                self.input.to_uppercase()
            }
            CmdType::Lowercase => {
                self.input.to_lowercase()
            }
            CmdType::Nospace => {
                self.input.replace(" ", "")
            }
            CmdType::Slugify => {
                slugify(&self.input)
            }
            CmdType::Csv => {
                self.input.split(',')
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(";") // TODO - later
            }
            CmdType::Error => {
                self.result.as_str().to_string()
            }
            CmdType::Exit => {
                std::process::exit(0);
            }
        }
    }
}