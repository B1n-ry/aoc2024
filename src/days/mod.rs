pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
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
        01 => day_01::run(file_input),
        02 => day_02::run(file_input),
        03 => day_03::run(file_input),
        04 => day_04::run(file_input),
        05 => day_05::run(file_input),
        06 => day_06::run(file_input),
        07 => day_07::run(file_input),
        08 => day_08::run(file_input),
        09 => day_09::run(file_input),
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
