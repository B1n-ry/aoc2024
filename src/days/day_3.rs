pub fn run(file_input: &str) {

    println!("Problem 1: {}", find_multiples(file_input, false));
    println!("Problem 2: {}", find_multiples(file_input, true));
}

fn find_multiples(file_input: &str, enable_use_flag: bool) -> i32 {
    let mut use_flag = true;

    file_input.split("mul(").map(|after_mul| {
        let res = get_multiplication(after_mul, use_flag);
        if enable_use_flag {
            let modified = format!("{}___", after_mul);
            modified.as_bytes().to_vec().windows(7).for_each(|arr| {
                if arr.starts_with(b"do()") {  // Starts with `do()`
                    use_flag = true;
                }
                if arr.starts_with(b"don't()") {  // Starts with `don't()`
                    use_flag = false;
                }
            });
        };
        res
    }).sum::<i32>()
}

fn get_multiplication(after_mul: &str, use_flag: bool) -> i32 {
    let Some((numbers, _)) = after_mul.split_once(')') else { return 0; };
    let Some((left, right)) = numbers.split_once(',') else { return 0; };
    if !left.chars().all(|c| c.is_digit(10)) || !right.chars().all(|c| c.is_digit(10)) { return 0; }

    if use_flag {
        left.parse::<i32>().expect("Failed to get number a second time") * right.parse::<i32>().expect("Failed to get number a second time")
    } else {
        0
    }
}
