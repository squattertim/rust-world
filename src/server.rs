use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use crate::client::messages::{deserialize_message, MessageType};
use crate::client::messages::MessageType::Text;
use crate::handle_error;

fn handle_client(mut stream: TcpStream) -> MessageType {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes).unwrap();
    let len = u32::from_be_bytes(len_bytes) as usize;

    let mut buffer = vec![0u8; len];
    stream.read_exact(&mut buffer).unwrap();

    deserialize_message(&buffer)
}

pub fn listen_and_accept(address: &str, mut clients: HashMap<SocketAddr, TcpStream>) {
    let listener = TcpListener::bind(address).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        clients.insert(addr.clone(), stream);

        let message = handle_client(clients.get(&addr).unwrap().try_clone().unwrap());
        // Here, you can further process this message as per your requirements

        match message {
            Text(mes) => {
                match mes.as_ref() {
                    "exit" => { std::process::exit(0) }
                    _ => { println!("{:?}", mes) }
                }
            }
            MessageType::File(name, cont) => {
                let mut file = fs::File::create(format!("{name}_transferred.png")).map_err(|err| {
                    handle_error(&format!("Error trying to create the file '{name}'"), err)
                }).unwrap();
                file.write_all(&*cont).expect("TODO: panic message");
            }
            _ => {}
        }
    }
}