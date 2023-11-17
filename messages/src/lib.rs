use serde::{Serialize, Deserialize};
use std::error::Error;
use std::process::exit;


#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Text(String),
    Image(Vec<u8>),
    File(String, Vec<u8>),  // Filename and its content as bytes
}

pub fn serialize_message(message: &MessageType) -> String {
    serde_json::to_string(&message).unwrap()
}

pub fn deserialize_message(data: &[u8]) -> MessageType {
    serde_json::from_slice(&data).unwrap()
}

pub fn handle_error(message: &str, err: impl Error) {
    exit_with_error_msg(&format!("{message}: {err}"));
}

pub fn exit_with_error_msg(message: &str) {
    eprintln!("{message}");
    exit(1);
}
