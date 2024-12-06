use std::{collections::{hash_map::Entry, HashMap, HashSet}, fmt::Display, ops::AddAssign, usize};

struct Grid<T> {
    grid: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.grid.get(y).map_or(None, |o| o.get(x))
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.grid.get_mut(y).map_or(None, |o| o.get_mut(x))
    }
    fn set(&mut self, x: usize, y: usize, replacement: T) {
        let Some(old) = self.get_mut(x, y) else { return; };
        *old = replacement;
    }
    fn find_first(&self, pred: impl Fn(&T) -> bool) -> Option<(usize, usize)> {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if pred(&value) {
                    return Some((x, y));
                }
            }
        }
        None
    }
}
impl Grid<char> {
    /// Find based on a set of coordinates and a direction if loop will form
    /// Returns weather or not the path is a loop, and all coordinates (bundled with direction) is in said path
    fn will_loop_from(pos: (usize, usize), direction: Direction, known_loops: &HashSet<(usize, usize, Direction)>) -> (bool, HashSet<(usize, usize, Direction)>) {
        let (x, y) = pos;
        if known_loops.contains(&(x, y, direction)) {

        }
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        let (width, height) = (
            value.get(0).map_or(0, |v| v.len()),
            value.len()
        );

        Grid { grid: value, width, height }
    }
}
impl<T> FromIterator<Vec<T>> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = Vec<T>>>(iter: I) -> Self {
        let g: Vec<Vec<T>> = iter.into_iter().collect();
        Grid::from(g)
    }
}
impl<T> Display for Grid<T> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid.iter().map(|row| row.iter().map(|value| format!("{}", value)).collect::<String>()).collect::<Vec<String>>().join("\n"))
    }
}

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
enum Direction {
    Left, Right, Down, Up,
}
impl Direction {
    fn step_from(&self, pos: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Left => (pos.0.overflowing_sub(1).0, pos.1),
            Self::Right => (pos.0.overflowing_add(1).0, pos.1),
            Self::Up => (pos.0, pos.1.overflowing_sub(1).0),
            Self::Down => (pos.0, pos.1.overflowing_add(1).0),
        }
    }
    fn turn_90_right(&mut self) {
        *self = self.rotated_90_right();
    }
    fn rotated_90_right(&self) -> Direction {
        match self {
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Right => Self::Down,
        }
    }
}

pub fn run(file_input: &str) {
    let mut guard_dir = Direction::Up;
    let grid: Grid<char> = file_input.lines().map(|l| l.chars().collect()).collect();
    let (mut guard_x, mut guard_y) = grid.find_first(|&c| c == '^').expect("Error: No guard (facing up) was found!");

    let mut explored = 0;  // Start is explored
    let mut ways_to_limbo = 0;

    let mut already_visited: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();

    while let Some((next_char, x, y)) = {
        let (x, y) = guard_dir.step_from((guard_x, guard_y));
        grid.get(x, y).map(|&c| (c, x, y))
    } {
        if next_char == '#' {
            guard_dir.turn_90_right();
            continue;
        }

        let guard_pos = (guard_x, guard_y);
        if let Some(_) = already_visited.get(&guard_pos) {
            // If a 90 degree right turn would make us repeat a cycle, AND we could have gotten to where we're at currently
            // meaning where we would place the stone we would not have walked over
        } else {
            explored.add_assign(1);
        }
        let mut turned_dir = guard_dir.rotated_90_right();
        let mut turned_pos = (guard_x, guard_y);
        let mut visited_clone = already_visited.clone();
        while let Some((next_char, x, y)) = {
            let (x, y) = turned_dir.step_from(turned_pos);
            grid.get(x, y).map(|&c| (c, x, y))
        } {
            if next_char == '#' {
                turned_dir.turn_90_right();
                continue;
            }
            if visited_clone.get(&turned_pos).is_some_and(|v| v.contains(&turned_dir)) {
                ways_to_limbo.add_assign(1);
                break;
            }
            match visited_clone.get_mut(&turned_pos) {
                Some(v) => { v.push(turned_dir); },
                None => { visited_clone.insert(guard_pos, vec![turned_dir]); },
            }
            turned_pos = (x, y);
        }

        match already_visited.get_mut(&guard_pos) {
            Some(v) => { v.push(guard_dir); },
            None => { already_visited.insert(guard_pos, vec![guard_dir]); },
        }
        (guard_x, guard_y) = (x, y);
    }
    if !already_visited.contains_key(&(guard_x, guard_y)) {
        explored.add_assign(1);
    }

    // println!("{}", grid);

    println!("Problem 1: {}", explored);
    println!("Problem 2: {}", ways_to_limbo);
}
