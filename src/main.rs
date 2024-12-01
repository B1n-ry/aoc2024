use std::{fs::File, io::Read, path::Path, time::Instant};

use days::*;

mod days;

const DAY: u16 = 1;
const TIMER_ACTIVE: bool = false;

fn main() {
    let path_str = format!("res/day_{}.txt", DAY);
    let path: &Path = Path::new(path_str.as_str());
    
    let Ok(mut file) = File::open(path) else {
        panic!("Could not open file for day {}", DAY);
    };

    let mut file_string: String = String::new();
    file.read_to_string(&mut file_string).expect("Failed to read file to string");

    if file_string.chars().all(|c| c.is_ascii_whitespace()) {
        panic!("res/day_{}.txt is empty!", DAY);
    }

    let now = Instant::now();

    // Functions and arguments start with an underscore to not give compiler warnings
    match DAY {
        1  =>  day_1::run(&file_string),
        2  =>  day_2::run(&file_string),
        3  =>  day_3::run(&file_string),
        4  =>  day_4::run(&file_string),
        5  =>  day_5::run(&file_string),
        6  =>  day_6::run(&file_string),
        7  =>  day_7::run(&file_string),
        8  =>  day_8::run(&file_string),
        9  =>  day_9::run(&file_string),
        10 => day_10::run(&file_string),
        11 => day_11::run(&file_string),
        12 => day_12::run(&file_string),
        13 => day_13::run(&file_string),
        14 => day_14::run(&file_string),
        15 => day_15::run(&file_string),
        16 => day_16::run(&file_string),
        17 => day_17::run(&file_string),
        18 => day_18::run(&file_string),
        19 => day_19::run(&file_string),
        20 => day_20::run(&file_string),
        21 => day_21::run(&file_string),
        22 => day_22::run(&file_string),
        23 => day_23::run(&file_string),
        24 => day_24::run(&file_string),
        25 => day_25::run(&file_string),

        0 => panic!("There is no 0th of December!"),
        _ => panic!("Christmas has passed :(")
    }

    if TIMER_ACTIVE {
        let elapsed = now.elapsed();
        let nanos = elapsed.as_nanos();
        if elapsed.as_secs() > 1 {
            println!("Day {} took {} s", DAY, elapsed.as_secs_f64());
        }
        else if elapsed.as_millis() > 1 {
            println!("Day {} took {} ms", DAY, nanos as f64 / 1_000_000_f64);
        }
        else if elapsed.as_micros() > 1 {
            println!("Day {} took {} us", DAY, nanos as f64 / 1_000_f64);
        }
        else {
            println!("Day {} took {} ns", DAY, nanos);
        }
    }
}
