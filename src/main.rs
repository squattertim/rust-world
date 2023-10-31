mod commands;

use std::{io, thread, sync::mpsc, str::FromStr};
use crate::commands::Command;
use crate::commands::CmdType;

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    thread::spawn(move || {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
                [ref _command] => {
                    tx.send(Command {
                        cmd: CmdType::Error,
                        input: "".to_string(),
                        result: "Please, pass  arguments in the form <command> <input>!".to_string(),
                    }).unwrap();
                }
                [ref _command, ref _input] => {
                    let cmd = Command::from_str(_command);
                    match cmd {
                        Ok(mut res) => {
                            res.input = _input.to_string();
                            tx.send(res).unwrap();
                        }
                        Err(_e) => {
                            tx.send(Command {
                                cmd: CmdType::Error,
                                input: _input.to_string(),
                                result: format!("Wrong command: {}", _command)
                            }).unwrap();
                        }
                    }
                }
                _ => {
                    tx.send(Command {
                        cmd: CmdType::Error,
                        input: "".to_string(),
                        result: "Too few or too many args!".to_string(),
                    }).unwrap();
                }
            }
        }
    });

    thread::spawn(move || {
        for mut received in rx {
            received.process();
            println!("{}", received.result());
        }
    }).join().unwrap();
}
