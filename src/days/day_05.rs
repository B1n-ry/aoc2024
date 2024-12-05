use std::{cmp::Ordering, collections::HashMap};

pub fn run(file_input: &str) {
    let (order_rules, orderings) = file_input.split_once("\r\n\r\n").expect("File format wrong! Didn't find double new-line");

    let mut is_before_map: HashMap<&str, Vec<&str>> = HashMap::new();
    order_rules.lines().for_each(|line| {
        let (before, after) = line.split_once("|").expect("Format wrong! Line didn't include a pipe (|) character!");
        match is_before_map.get_mut(before) {
            Some(v) => { v.push(after); },
            None => { is_before_map.insert(before, vec![after]); },
        };
    });

    let mut incorrect_orderings: Vec<&str> = Vec::new();

    let correct_orderings: Vec<Vec<&str>> = orderings.lines().filter_map(|line| {
        let current_order: Vec<&str> = line.split(',').collect();
        if current_order.is_sorted_by(|a, b| order_entries(a, b, &is_before_map).is_le()) {
            Some(current_order)
        } else {
            incorrect_orderings.push(line);
            None
        }
    }).collect();

    let fixed_orderings: Vec<Vec<&str>> = incorrect_orderings.iter().map(|line| {
        let mut current_order: Vec<&str> = line.split(',').collect();
        current_order.sort_by(|a, b| order_entries(a, b, &is_before_map));

        current_order
    }).collect();

    let p1 = sum_middles(&correct_orderings);
    let p2 = sum_middles(&fixed_orderings);

    println!("Problem 1: {}", p1);
    println!("Problem 2: {}", p2);
}

fn order_entries(a: &str, b: &str, is_before_map: &HashMap<&str, Vec<&str>>) -> Ordering {
    let Some(cant_be_after) = is_before_map.get(a) else {
        return Ordering::Greater;
    };
    if cant_be_after.contains(&b) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}
fn sum_middles(v: &Vec<Vec<&str>>) -> i32 {
    v.iter().map(|v| {
        let middle_index = v.len() / 2;
        v[middle_index].parse::<i32>().expect("Format wrong! Could not find a number!")
    }).sum::<i32>()
}
