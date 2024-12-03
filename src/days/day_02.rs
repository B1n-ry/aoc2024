use std::{collections::HashMap, i32, ops::AddAssign};

pub fn run(file_input: &str) {
    let mut safe1 = 0;
    let mut safe2 = 0;
    
    file_input.lines().for_each(|line| {
        let sequence = line.split_ascii_whitespace().map(|s| s.parse::<i32>().expect("Wrong format detected. Not a number")).collect::<Vec<i32>>();
        safe1 += is_safe(&sequence);
        safe2 += is_safe_with_removed(&sequence) | is_safe(&sequence);
    });

    println!("Problem 1: {}", safe1);
    println!("Problem 2: {}", safe2);
}

fn is_safe(sequence: &Vec<i32>) -> i32 {
    let mut differences: HashMap<i32, i32> = HashMap::new();
    
    sequence.windows(2).for_each(|window| {
        let difference = window[0] - window[1];
        match differences.get_mut(&difference) {
            Some(instances) => { instances.add_assign(1); },
            None => { differences.insert(difference, 1); },
        };
    });
    
    (
        differences.keys().all(|key| [1, 2, 3].contains(key))
        || differences.keys().all(|key| [-1, -2, -3].contains(key))
    ) as i32
}

fn is_safe_with_removed(sequence: &Vec<i32>) -> i32 {
    (0..sequence.len()).any(|i| {
        let mut seq_copy = sequence.clone();
        seq_copy.remove(i);
        
        is_safe(&seq_copy) == 1
    }) as i32
}
