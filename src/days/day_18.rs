use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

const MAX_COORDINATE: usize = 70;

#[derive(PartialEq, Eq)]
struct Node(u32, usize, usize);
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(file_input: &str) {
    let full_bit_list: Vec<(usize, usize)> = file_input.lines().map(|line| {
        let (x, y) = line.split_once(',').expect("Format wrong! Line does not include comma sign!");
        (x.parse().expect("Format error! Not number!"), y.parse().expect("Format error! Not number!"))
    }).collect();

    let mut steps = get_fastest_way(&full_bit_list[..1024]);

    println!("Problem 1: {}", steps.len() - 1);  // Ignore first step

    for i in 1024..full_bit_list.len() {
        let (x, y) = full_bit_list[i];
        if steps.contains(&(x, y)) {
            steps = get_fastest_way(&full_bit_list[..=i]);
            if !steps.contains(&(0, 0)) {
                println!("Problem 2: {},{}", x, y);
                break;
            }
        }
    }
}

fn get_fastest_way(obstacles: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut costs = HashMap::from([((0, 0), 0)]);
    let mut visited = HashSet::new();
    let mut previous = HashMap::new();

    let mut queue = BinaryHeap::from([Node(0, 0, 0)]);
    while let Some(Node(_, x, y)) = queue.pop() {
        let &here_cost = costs.get(&(x, y)).expect("Encountered semi-visited cell without cost");

        for &(neighbour_x, neighbour_y) in &get_neighbours(x, y) {
            if obstacles.contains(&(neighbour_x, neighbour_y)) { continue; }

            let next_cost = here_cost + 1;
            if costs.get(&(neighbour_x, neighbour_y)).is_none_or(|&cost| cost > next_cost) {
                queue.push(Node(next_cost + heuristic(x, y), neighbour_x, neighbour_y));
                previous.insert((neighbour_x, neighbour_y), (x, y));
                costs.insert((neighbour_x, neighbour_y), next_cost);
            }
        }
        visited.insert((x, y));
    }

    let mut next = (MAX_COORDINATE, MAX_COORDINATE);
    let mut path = VecDeque::from([next]);
    while let Some(&(prev_x, prev_y)) = previous.get(&next) {
        next = (prev_x, prev_y);
        path.push_front(next);
    }
    path.into()
}

fn get_neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    if let (Some(x), y) = (x.checked_add(1), y) {
        if is_within_bounds(x, y) { v.push((x, y)); }
    }
    if let (x, Some(y)) = (x, y.checked_add(1)) {
        if is_within_bounds(x, y) { v.push((x, y)); }
    }
    if let (Some(x), y) = (x.checked_sub(1), y) {
        if is_within_bounds(x, y) { v.push((x, y)); }
    }
    if let (x, Some(y)) = (x, y.checked_sub(1)) {
        if is_within_bounds(x, y) { v.push((x, y)); }
    }
    v
}

fn is_within_bounds(x: usize, y: usize) -> bool {
    (0..=MAX_COORDINATE).contains(&x) && (0..=MAX_COORDINATE).contains(&y)
}

fn heuristic(x: usize, y: usize) -> u32 {
    ((MAX_COORDINATE - x) + (MAX_COORDINATE - y)) as u32
}
