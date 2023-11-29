use chrono::{Datelike, Local, Timelike};
use std::env;

fn moon_phase(year: i32, month: u32, day: u32) -> usize {
    let mut d = day;
    if month == 2 {
        d += 31;
    } else if month > 2 {
        d += 59 + ((month as f64 - 3.0) * 30.6 + 0.5) as u32;
    }
    let g = (year - 1900) % 19;
    let mut e = (11 * g + 29) % 30;
    if e == 25 || e == 24 {
        e += 1;
    }
    ((((e + d) * 6 + 5) % 177) / 22 & 7) as usize
}

fn main() {
    let phase = [
        "waxing crescent", "at first quarter",
        "waxing gibbous", "full", "waning gibbous",
        "at last quarter", "waning crescent", "new"
    ];

    let now = Local::now();
    let (year, month, day) = (now.year(), now.month(), now.day());
    let time_string = now.format("Today is %A, %B %d, %Y\nIt is %r\n").to_string();

    let args: Vec<String> = env::args().collect();
    print!("Greetings");
    if args.len() > 1 {
        print!(", {}", args[1]);
    }
    println!("!\n{}", time_string);
    let mp = moon_phase(year, month, day);
    println!("The moon is {}", phase[mp]);
}