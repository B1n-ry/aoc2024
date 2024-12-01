pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;
pub mod day_8;
pub mod day_9;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;
pub mod day_19;
pub mod day_20;
pub mod day_21;
pub mod day_22;
pub mod day_23;
pub mod day_24;
pub mod day_25;
 
pub fn run_one(file_input: &str, day: u16) {
    match day {
        1 => day_1::run(file_input),
        2 => day_2::run(file_input),
        3 => day_3::run(file_input),
        4 => day_4::run(file_input),
        5 => day_5::run(file_input),
        6 => day_6::run(file_input),
        7 => day_7::run(file_input),
        8 => day_8::run(file_input),
        9 => day_9::run(file_input),
        10 => day_10::run(file_input),
        11 => day_11::run(file_input),
        12 => day_12::run(file_input),
        13 => day_13::run(file_input),
        14 => day_14::run(file_input),
        15 => day_15::run(file_input),
        16 => day_16::run(file_input),
        17 => day_17::run(file_input),
        18 => day_18::run(file_input),
        19 => day_19::run(file_input),
        20 => day_20::run(file_input),
        21 => day_21::run(file_input),
        22 => day_22::run(file_input),
        23 => day_23::run(file_input),
        24 => day_24::run(file_input),
        25 => day_25::run(file_input),

        0 => panic!("There is no 0th of December!"),
        _ => panic!("Christmas has passed :("),
    };
}
