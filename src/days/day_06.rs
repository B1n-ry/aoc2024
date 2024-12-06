use std::{collections::HashSet, fmt::Display, ops::AddAssign};

#[allow(unused)]
struct Grid<T> {
    grid: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    #[allow(unused)]
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.grid.get(y).map_or(None, |o| o.get(x))
    }
    #[allow(unused)]
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.grid.get_mut(y).map_or(None, |o| o.get_mut(x))
    }
    #[allow(unused)]
    fn set(&mut self, x: usize, y: usize, replacement: T) {
        let Some(old) = self.get_mut(x, y) else { return; };
        *old = replacement;
    }
    #[allow(unused)]
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
    fn will_loop_from(&self, pos: (usize, usize), direction: Direction) -> bool {
        let (mut x, mut y) = pos;
        let mut current_dir = direction;
        let mut path_includes = HashSet::new();

        while let Some((next_char, next_x, next_y)) = {
            let (next_x, next_y) = current_dir.step_from((x, y));
            self.get(next_x, next_y).map(|&c| (c, next_x, next_y))
        } {
            if next_char == '#' {
                current_dir.turn_90_right();
                continue;
            }

            if path_includes.contains(&(x, y, current_dir)) {
                return true;
            }
            path_includes.insert((x, y, current_dir));

            (x, y) = (next_x, next_y);
        }

        false
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
    let mut grid: Grid<char> = file_input.lines().map(|l| l.chars().collect()).collect();
    let (mut guard_x, mut guard_y) = grid.find_first(|&c| c == '^').expect("Error: No guard (facing up) was found!");

    let mut explored = 0;  // Start is explored
    let mut ways_to_limbo = 0;

    let mut already_visited: HashSet<(usize, usize)> = HashSet::new();
    let mut obstruction_positions: HashSet<(usize, usize)> = HashSet::new();

    while let Some((next_char, x, y)) = {
        let (x, y) = guard_dir.step_from((guard_x, guard_y));
        grid.get(x, y).map(|&c| (c, x, y))
    } {
        if next_char == '#' {
            guard_dir.turn_90_right();
            continue;
        }

        if let Some(&cached) = grid.get(x, y) {
            if !already_visited.contains(&(x, y)) {
                grid.set(x, y, '#');
                let looping = grid.will_loop_from((guard_x, guard_y), guard_dir.rotated_90_right());
                grid.set(x, y, cached);

                if looping {
                    if !obstruction_positions.contains(&(x, y)) {
                        ways_to_limbo.add_assign(1);
                    }
                    obstruction_positions.insert((x, y));
                }
            }
        }


        let guard_pos = (guard_x, guard_y);
        if !already_visited.contains(&guard_pos) {
            explored.add_assign(1);
        }

        already_visited.insert(guard_pos);
        (guard_x, guard_y) = (x, y);
    }
    if !already_visited.contains(&(guard_x, guard_y)) {
        explored.add_assign(1);
    }

    println!("Problem 1: {}", explored);
    println!("Problem 2: {}", ways_to_limbo);
}
