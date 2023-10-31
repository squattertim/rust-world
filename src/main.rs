use std::{io, thread, sync::mpsc};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            tx.send(input).unwrap();
        }
    });

    thread::spawn(move || {
        for received in rx {
            println!("{}", received);
        }
    }).join().unwrap();
}
