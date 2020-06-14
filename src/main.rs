use std::io::{self, Read, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

struct Timer {
    total: Instant,
    line: Instant,
    use_line_timer: bool,
}

impl Timer {
    pub fn new(use_line_timer: bool) -> Timer {
        Timer {
            total: Instant::now(),
            line: Instant::now(),
            use_line_timer
        }
    }

    pub fn reset_line_timer(&mut self) {
        self.line = Instant::now();
    }

    pub fn get_timestamp(&self) -> String {
        if self.use_line_timer {
            format!("[{}|{}]", Self::format_time(self.total), Self::format_time(self.line))
        } else {
            format!("[{}]", Self::format_time(self.total))
        }
    }

    fn format_time(instant: Instant) -> String {
        let millis = instant.elapsed().as_millis();
        let mut ss = millis / 1000;
        let mut mm = ss / 60;
        ss %= 60;
        let hh = mm / 60;
        mm %= 60;
        let mut res = String::new();
        if hh > 0 {
            res += &format!("{:02}:", hh);
        }
        res += &format!("{:02}:{:02}", mm, ss);
        if hh == 0 {
            res += &format!(".{:02}", millis % 1000 / 10);
        }
        res
    }
}

fn main() {
    // TODO: implement proper argument parsing
    let mut opt_use_line_timers = false;
    match std::env::args().len() {
        1 => {}
        2 => match std::env::args().nth(1) {
            Some(s) if s == "-l" => {
                opt_use_line_timers = true;
            }
            _ => {
                println!("ticktock: invalid arguments");
                return;
            }
        }
        _ => {
            println!("ticktock: invalid arguments");
            return;
        }
    }

    let mut timer = Timer::new(opt_use_line_timers);
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    thread::spawn(move || loop {
        let mut v = vec![0; 4096];
        let size = io::stdin().read(&mut v).unwrap();
        if size == 0 {
            return;
        }
        tx.send(Vec::from(&v[..size])).unwrap();
    });

    print!("{}", timer.get_timestamp());
    print!("\x1b[7l");
    io::stdout().flush().unwrap();

    let mut line = String::new();
    loop {
        match rx.try_recv() {
            Ok(v) => {
                let mut new_data = &v[..];
                while !new_data.is_empty() {
                    match new_data.iter().position(|&c| c == '\n' as u8) {
                        Some(i) => {
                            line += &String::from_utf8(Vec::from(&new_data[..i])).unwrap();
                            println!("\x1b[G{} {}", timer.get_timestamp(), line);
                            line = String::new();
                            new_data = &new_data[i + 1..];
                            timer.reset_line_timer();
                        }
                        None => {
                            line += &String::from_utf8(Vec::from(new_data)).unwrap();
                            print!("\x1b[G{} {}", timer.get_timestamp(), line);
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
        print!("\x1b[s"); // cursor position save
        print!("\x1b[G{}", timer.get_timestamp());
        print!("\x1b[u"); // cursor position restore
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(15));
    }

    print!("\x1b[7h");
    io::stdout().flush().unwrap();
}
