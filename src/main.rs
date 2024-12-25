use std::{fs::File, io::Read, path::Path, time::Instant};

mod days;

const DAY: u16 = 20;
const TIMER_ACTIVE: bool = false;

fn main() {
    let path_str = format!("res/day_{:02}.txt", DAY);
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

    days::run_one(&file_string, DAY);

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
