mod commands;

use std::{io, thread, sync::mpsc, str::FromStr};
use crate::commands::{Command};
use crate::commands::CmdType;

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    thread::spawn(move || {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            match input.split_whitespace().collect::<Vec<&str>>().as_slice() {
                [_command] => {
                    tx.send(match_cmd(_command, "".to_string())).unwrap();
                }
                [_command, _input @ ..] => {
                    tx.send(match_cmd(_command, _input.join(" ").to_string())).unwrap();
                }
                _ => {
                    tx.send(Command {
                        cmd: CmdType::Error,
                        input: "".to_string(),
                        result: "Too few args!".to_string(),
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

fn match_cmd(_command: &&str, input: String) -> Command {
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
                result: format!("Wrong command: {}", _command),
            }
        }
    }
}
