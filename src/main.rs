use std::{fs::File, io::Read, path::Path};

mod days;

const DAY: u16 = 1;

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

    // Functions and arguments start with an underscore to not give compiler warnings
    match DAY {
        1 => days::day_1::_run(&file_string),
        2 => days::day_1::_run(&file_string),
        3 => days::day_1::_run(&file_string),
        4 => days::day_1::_run(&file_string),
        5 => days::day_1::_run(&file_string),
        6 => days::day_1::_run(&file_string),
        7 => days::day_1::_run(&file_string),
        8 => days::day_1::_run(&file_string),
        9 => days::day_1::_run(&file_string),
        10 => days::day_1::_run(&file_string),
        11 => days::day_1::_run(&file_string),
        12 => days::day_1::_run(&file_string),
        13 => days::day_1::_run(&file_string),
        14 => days::day_1::_run(&file_string),
        15 => days::day_1::_run(&file_string),
        16 => days::day_1::_run(&file_string),
        17 => days::day_1::_run(&file_string),
        18 => days::day_1::_run(&file_string),
        19 => days::day_1::_run(&file_string),
        20 => days::day_1::_run(&file_string),
        21 => days::day_1::_run(&file_string),
        22 => days::day_1::_run(&file_string),
        23 => days::day_1::_run(&file_string),
        24 => days::day_1::_run(&file_string),
        25 => days::day_1::_run(&file_string),

        0 => panic!("There is no 0th of December!"),
        _ => panic!("Christmas has passed :(")
    }
}
