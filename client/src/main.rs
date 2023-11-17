mod client;
mod commands;

use std::{io, thread};
use crate::client::send_message;
use crate::commands::match_cmd;

fn main() {
    let address = "127.0.0.1:7878";

    thread::spawn(move || {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
                [_command] => {
                    //send_message(address, &MessageType::Text(_command.to_string()));
                    send_message(address, &match_cmd(_command, _command.to_string()).process().unwrap());
                }
                [_command, _input @ ..] => {
                    send_message(address, &match_cmd(_command, _input.join(" ").to_string()).process().unwrap());
                }
                _ => {
                    println!("{}", "Too few args!".to_string());
                }
            }
        }
    }).join().unwrap()
}

