pub fn run(file_input: &str) {

    println!("Problem 1: {}", find_multiples(file_input, false));
    println!("Problem 2: {}", find_multiples(file_input, true));
}

fn find_multiples(file_input: &str, enable_use_flag: bool) -> i32 {
    let mut use_flag = true;

    (0..file_input.len()).map(|i| &file_input[i..]).map(|slice| {
        if enable_use_flag {
            if slice.starts_with("do()") { use_flag = true; }
            if slice.starts_with("don't()") { use_flag = false; }
        }
        if slice.starts_with("mul(") {
            let str_iterator = &mut slice[4..].chars();
            let mut last_char = None;
            let left_num = str_iterator.take_while(|&c| { last_char = Some(c); c.is_ascii_digit() }).collect::<String>();
            if left_num.is_empty() || last_char != Some(',') {
                return 0;
            }
            let mut last_char = None;
            let right_num = str_iterator.take_while(|&c| { last_char = Some(c); c.is_ascii_digit() }).collect::<String>();
            if right_num.is_empty() || last_char != Some(')') {
                return 0;
            }

            if use_flag {
                return left_num.parse::<i32>().ok().zip(right_num.parse::<i32>().ok()).map(|(right, left)| right * left).expect("Failed to get number a second time")
            }
        }
        0
    }).sum()
}
