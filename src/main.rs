use std::io::{self, Read, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

fn format_time(millis: u128) -> String {
    let mut ss = millis / 1000;
    let mut mm = ss / 60;
    ss %= 60;
    let hh = mm / 60;
    mm %= 60;
    let mut res = String::new();
    res += "[";
    if hh > 0 {
        res += &format!("{:02}:", hh);
    }
    res += &format!("{:02}:{:02}", mm, ss);
    if hh == 0 {
        res += &format!(".{:02}", millis % 1000 / 10);
    }
    res += "]";
    res
}

fn main() {
    let time_total = Instant::now();
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    thread::spawn(move || loop {
        let mut v = vec![0; 4096];
        let size = io::stdin().read(&mut v).unwrap();
        if size == 0 {
            return;
        }
        tx.send(Vec::from(&v[..size])).unwrap();
    });
    let mut line = String::new();
    print!("\x1b[s\x1b[E{}\x1b[u", format_time(time_total.elapsed().as_millis()));
    io::stdout().flush().unwrap();
    loop {
        match rx.try_recv() {
            Ok(v) => {
                let mut new_data = &v[..];
                while !new_data.is_empty() {
                    match new_data.iter().position(|&c| c == '\n' as u8) {
                        Some(i) => {
                            line += &String::from_utf8(Vec::from(&new_data[..i])).unwrap();
                            println!("\x1b[G{} {}", format_time(time_total.elapsed().as_millis()), line);
                            line = String::new();
                            new_data = &new_data[i + 1..];
                        }
                        None => {
                            line += &String::from_utf8(Vec::from(new_data)).unwrap();
                            print!("\x1b[G{} {}", format_time(time_total.elapsed().as_millis()), line);
                            io::stdout().flush().unwrap();
                            new_data = &new_data[new_data.len()..];
                        }
                    }
                }
            }
            Err(TryRecvError::Empty) => {}
            Err(TryRecvError::Disconnected) => {
                return;
            }
        }
        print!("\x1b[s\x1b[E{}\x1b[u", format_time(time_total.elapsed().as_millis()));
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(15));
    }
}
