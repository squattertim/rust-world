use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
pub struct Command {
    pub(crate) command: String,
    pub(crate) input: String,
    pub(crate) result: String
}

unsafe impl Send for Command {}

impl FromStr for Command {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { command: String::from(s), input: "".to_string(), result: "".to_string() })
    }
}