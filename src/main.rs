use std::time::{Duration, Instant};
use std::thread;

fn format_time(millis: u128) -> String {
    let mut ss = millis / 1000;
    let mut mm = ss / 60;
    ss %= 60;
    let hh = mm / 60;
    mm %= 60;
    let mut res = String::new();
    res += "[+";
    if hh > 0 {
        res += &format!("{:02}:", hh);
    }
    res += &format!("{:02}:{:02}.{:02}]", mm, ss, millis % 1000 / 10);
    res
}

fn main() {
    let start = Instant::now();
    loop {
        println!("{}\x1b[1A", format_time(start.elapsed().as_millis()));
        thread::sleep(Duration::new(0, 15000000));
    }
}
