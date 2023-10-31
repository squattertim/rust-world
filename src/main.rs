mod commands;

use std::{io, thread, sync::mpsc, str::FromStr};
use crate::commands::Command;

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    thread::spawn(move || {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
                [ref command] => {
                    tx.send(Command {
                        command: "".to_string(),
                        input: "".to_string(),
                        result: "Please, pass  arguments in the form <command> <input>!".to_string(),
                    }).unwrap();
                }
                [ref command, ref string] => {
                    let cmd = Command::from_str(command);
                    match cmd {
                        Ok(res) => {
                            tx.send(res).unwrap();
                        }
                        _ => {}
                    }
                }
                _ => {
                    tx.send(Command {
                        command: "".to_string(),
                        input: "".to_string(),
                        result: "Too many args!".to_string(),
                    }).unwrap();
                }
            }
        }
    });

    thread::spawn(move || {
        for received in rx {
            println!("{}", received.result);
        }
    }).join().unwrap();
}
