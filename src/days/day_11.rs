use {std::ops::Mul,memoize::memoize};

pub fn run(file_input: &str) {
    let stones: Vec<usize> = file_input.split_ascii_whitespace().map(|s| s.parse().expect("Wrong format! Not a number!")).collect();

    let stone_count_25: usize = stones.iter().map(|&stone| get_stones_in_blinks(stone, 25)).sum();
    let stone_count_75: usize = stones.iter().map(|&stone| get_stones_in_blinks(stone, 75)).sum();

    println!("Problem 1: {}", stone_count_25);
    println!("Problem 2: {}", stone_count_75);
}

#[memoize]
fn get_stones_in_blinks(og_stone: usize, blinks_remaining: usize) -> usize {
    if blinks_remaining == 0 { return 1; }

    let next_stones = get_next_stones_from(og_stone);
    next_stones.iter().map(|&stone| get_stones_in_blinks(stone, blinks_remaining - 1)).sum()
}

fn get_next_stones_from(stone: usize) -> Vec<usize> {
    if stone == 0 {
        return vec![1];
    }

    let digit_length = stone.to_string().len();
    if digit_length % 2 == 0 {
        let divisor = 10_usize.pow(digit_length as u32 / 2);
        return vec![stone / divisor, stone % divisor];
    }
    vec![stone.mul(2024)]
}
