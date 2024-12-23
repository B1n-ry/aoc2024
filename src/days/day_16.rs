use std::{collections::{BinaryHeap, HashMap, HashSet}, fmt::Display, ops::Add};

#[allow(dead_code)]
struct Grid<T> {
    grid: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl FromIterator<Vec<char>> for Grid<char> {
    fn from_iter<T: IntoIterator<Item = Vec<char>>>(iter: T) -> Self {
        let grid: Vec<Vec<char>> = iter.into_iter().collect();
        let height = grid.len();
        let width = grid.get(0).map_or(0, |row| row.len());

        Self { grid, width, height }
    }
}

#[allow(unused)]
impl<T> Grid<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.grid.get(y).map_or(None, |v| v.get(x))
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.grid.get_mut(y).map_or(None, |v| v.get_mut(x))
    }
    fn set(&mut self, x: usize, y: usize, value: T) {
        if let Some(val) = self.get_mut(x, y) {
            *val = value;
        }
    }

    fn new(width: usize, height: usize, default: T) -> Self where T: Clone {
        Grid {
            grid: vec![vec![default; width]; height],
            width,
            height,
        }
    }

    fn get_all(&self, predicate: fn(c: &T) -> bool) -> Vec<(usize, usize)> {
        self.grid.iter().enumerate().flat_map(|(y, row)| row.iter().enumerate().filter_map(move |(x, col)| {
            if predicate(col) { Some((x, y)) } else { None }
        })).collect()
    }

    fn for_each(&self, mut func: impl FnMut(usize, usize, &T)) {
        self.grid.iter().enumerate().for_each(|(y, row)| row.iter().enumerate().for_each(|(x, c)| func(x, y, c)));
    }
}

impl<T> Display for Grid<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid.iter().map(|row| row.iter().map(|value| format!("{}", value)).collect::<String>()).collect::<Vec<String>>().join("\n"))
    }
}

#[derive(PartialEq, PartialOrd, Eq, Clone, Copy, Hash)]
enum Direction {
    East, West, North, South,
}
impl Direction {
    fn turn_90_deg(&self) -> [Self; 2] {
        match self {
            Direction::East | Direction::West => [Direction::North, Direction::South],
            Direction::North | Direction::South => [Direction::East, Direction::West]
        }
    }
}
impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);
    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::South => (self.0, self.1 + 1),
            Direction::West => (self.0 - 1, self.1),
            Direction::East => (self.0 + 1, self.1),
            Direction::North => (self.0, self.1 - 1),
        }
    }
}

#[derive(PartialEq, Eq)]
struct HeapEntry(u32, usize, usize, Direction, Option<(usize, usize, Direction)>);

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(file_input: &str) {
    let map: Grid<char> = file_input.lines().map(|line| line.chars().collect()).collect();
    let (start_x, start_y) = map.get_all(|&c| c == 'S')[0];
    let end = map.get_all(|&c| c == 'E')[0];

    let mut min_cost = u32::MAX;

    let mut visited_costs: HashMap<(usize, usize, Direction), u32> = HashMap::new();
    let mut previous_link: HashMap<(usize, usize, Direction), Vec<(usize, usize, Direction)>> = HashMap::new();
    let mut queue = BinaryHeap::from([HeapEntry(0, start_x, start_y, Direction::East, None)]);
    while let Some(HeapEntry(cost, x, y, direction, o_prev)) = queue.pop() {
        if visited_costs.get(&(x, y, direction)).is_some_and(|&c| cost > c) {
            continue;
        }

        if let Some(prev) = o_prev {
            match previous_link.get_mut(&(x, y, direction)) {
                Some(v) => {
                    if !v.contains(&prev) {
                        v.push(prev);
                    }
                },
                None => {
                    previous_link.insert((x, y, direction), vec![prev]);
                },
            }
        }

        if (x, y) == end {
            min_cost = min_cost.min(cost);
        }
        if cost > min_cost {
            break;
        }

        for d in direction.turn_90_deg() {
            let (turned_x, turned_y) = (x, y) + d;
            if map.get(turned_x, turned_y).is_some_and(|&c| c != '#') {
                queue.push(HeapEntry(cost + 1000, x, y, d, Some((x, y, direction))));
            }
        }

        let (candidate_x, candidate_y) = (x, y) + direction;
        if map.get(candidate_x, candidate_y).is_some_and(|&c| c != '#') {
            queue.push(HeapEntry(cost + 1, candidate_x, candidate_y, direction, Some((x, y, direction))));
        }

        visited_costs.insert((x, y, direction), cost);
    }

    let mut queue: Vec<(usize, usize, Direction)> = [Direction::East, Direction::West,
        Direction::North, Direction::South].iter().map(|&d| (end.0, end.1, d)).collect();
    
    let mut optimal_benches = HashSet::new();
    while let Some(last) = queue.pop() {
        optimal_benches.insert((last.0, last.1));
        let Some(prev) = previous_link.get(&last) else {
            continue;
        };
        for &e in prev {
            queue.push(e);
        }
    }

    println!("Problem 1: {}", min_cost);
    println!("Problem 2: {}", optimal_benches.len());
}
