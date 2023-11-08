mod client;
mod server;

use std::{fs, thread};
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::net::{SocketAddr, TcpStream};
use std::process::exit;
use crate::client::messages::MessageType;
use crate::client::send_message;
use crate::server::listen_and_accept;

fn main() {
    let address = "127.0.0.1:7878";
    let clients: HashMap<SocketAddr, TcpStream> = HashMap::new();

    thread::spawn(move || {
        send_message(address, &MessageType::Text("Hello World!".to_string()));

        let file_path = "cuddlyferris.png";
        let mut file = fs::File::open(&file_path.to_string()).map_err(|err| {
            handle_error(&format!("Error trying to open file '{file_path}'"), err);
        }).unwrap();

        let total_file_size = file.metadata().map_err(|err| {
            handle_error(&format!("Error trying to get size of file '{file_path}'"), err);
        }).unwrap().len();

        let mut buffer = vec![0; total_file_size as usize];
        file.read(&mut buffer).expect("buffer overflow");

        send_message(address, &MessageType::File(file_path.to_string(), buffer));
        send_message(address, &MessageType::Text("exit".to_string()));
    });

    thread::spawn(move || {
        listen_and_accept(address, clients)
    }).join().unwrap();
}

fn handle_error(message: &str, err: impl Error) {
    exit_with_error_msg(&format!("{message}: {err}"));
}

fn exit_with_error_msg(message: &str) {
    eprintln!("{message}");
    exit(1);
}
