use std::io::Write;
use std::net::TcpStream;
use crate::client::messages::{MessageType, serialize_message};

pub(crate) mod messages;

pub fn send_message(address: &str, message: &MessageType) {
    let serialized = serialize_message(message);
    let mut stream = TcpStream::connect(address).unwrap();

    // Send the length of the serialized message (as 4-byte value).
    let len = serialized.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();

    // Send the serialized message.
    stream.write_all(serialized.as_bytes()).unwrap();
}