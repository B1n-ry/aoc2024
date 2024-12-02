use std::i32;

pub fn run(file_input: &str) {
    let mut safe1 = 0;
    let mut safe2 = 0;
    
    file_input.lines().for_each(|line| {
        let sequence = line.split_ascii_whitespace().map(|s| s.parse::<i32>().expect("Wrong format detected. Not a number")).collect::<Vec<i32>>();
        safe1 += is_100_safe(&sequence);
        safe2 += is_safe_with_removed(&sequence);
    });

    println!("Problem 1: {}", safe1);
    println!("Problem 2: {}", safe2);
}

fn is_100_safe(sequence: &Vec<i32>) -> i32 {
    if !sequence.iter().zip(sequence.iter().skip(1)).all(|(&a1, &a2)| {
        a1.abs_diff(a2) <= 3
    }) {
        return 0;
    }

    (sequence.iter().try_fold(i32::MAX, |acc, &el| {
        if acc > el {
            Some(el)
        } else {
            None
        }
    }).is_some() || sequence.iter().try_fold(i32::MIN, |acc, &el| {
        if acc < el {
            Some(el)
        } else {
            None
        }
    }).is_some()) as i32
}

fn is_safe_with_removed(sequence: &Vec<i32>) -> i32 {
    for i in 0..sequence.len() {
        let mut seq_copy = sequence.clone();
        seq_copy.remove(i);
        if is_100_safe(&seq_copy) == 1 {
            return 1;
        }
    }
    0
}
