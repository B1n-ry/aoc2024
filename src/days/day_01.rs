use std::collections::BinaryHeap;

pub fn run(file_input: &str) {
    let mut left_col: BinaryHeap<i32> = BinaryHeap::new();
    let mut right_col: BinaryHeap<i32> = BinaryHeap::new();

    file_input.lines().for_each(|line| {
        let (left, right) = line.split_once("   ").expect("Wrong format detected (invalid space amount)");
        left_col.push(left.parse().expect("Format wrong! No number detected"));
        right_col.push(right.parse().expect("Format wrong! No number detected"));
    });

    // Clone because we loop through the heaps/trees exhaustively, emptying them through the process 
    prob1(left_col.clone(), right_col.clone());
    prob2(left_col.clone(), right_col.clone());
}

fn prob1(mut left_col: BinaryHeap<i32>, mut right_col: BinaryHeap<i32>) {
    let mut sum = 0;
    while let (Some(left), Some(right)) = (left_col.pop(), right_col.pop()) {
        sum += left.abs_diff(right);
    }

    println!("Problem 1: {}", sum);
}

fn prob2(mut left_col: BinaryHeap<i32>, right_col: BinaryHeap<i32>) {
    let mut sum = 0;
    while let Some(left) = left_col.pop() {
        sum += left * right_col.iter().fold(0, |acc, &el| acc + if el == left { 1 } else { 0 });
    }

    println!("Problem 2: {}", sum);
}
