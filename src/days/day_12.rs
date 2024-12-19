use std::{char, collections::HashSet, fmt::Display, str::Chars};

#[allow(dead_code)]
struct Grid<T> {
    grid: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<'a> FromIterator<Chars<'a>> for Grid<char> {
    fn from_iter<T: IntoIterator<Item = Chars<'a>>>(iter: T) -> Self {
        let grid: Vec<Vec<char>> = iter.into_iter().map(|i| i.collect()).collect();
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}

pub fn run(file_input: &str) {
    let grid: Grid<char> = file_input.lines().map(|line| line.chars()).collect();

    let mut explored: HashSet<(usize, usize)> = HashSet::new();

    let mut total_score = 0;
    let mut bulk_score = 0;

    grid.for_each(|x, y, c| {
        if explored.contains(&(x, y)) { return; }

        let mut borders = 0;
        let mut sized_borders = 0;
        let mut area = 0;

        let mut border_set = HashSet::new();

        let mut queue: Vec<(usize, usize)> = vec![(x, y)];
        while let Some((x, y)) = queue.pop() {
            if explored.contains(&(x, y)) { continue; }  // In case it's been added more than once without visit
            area += 1;
            let mut current_borders = vec![Direction::Up, Direction::Left, Direction::Right, Direction::Down];

            let neighbours = [
                (x.checked_add(1), Some(y), Direction::Right),
                (x.checked_sub(1), Some(y), Direction::Left),
                (Some(x), y.checked_add(1), Direction::Down),
                (Some(x), y.checked_sub(1), Direction::Up),
            ];
            for &(next_x, next_y, direction) in &neighbours {
                let (Some(next_x), Some(next_y)) = (next_x, next_y) else { continue; };
                if grid.get(next_x, next_y) == Some(c) {
                    if let Some(i) = current_borders.iter().position(|dir| dir == &direction) {
                        current_borders.remove(i);
                    } else {
                        println!("Somehow we didn't find direction");  // This should be impossible
                    }

                    if !explored.contains(&(next_x, next_y)) {
                        queue.push((next_x, next_y));
                    }
                }
            }

            let actual_neighbours: Vec<(usize, usize, Direction)> = neighbours.iter().filter_map(|&(x, y, d)| {
                let (Some(x), Some(y), d) = (x, y, d) else { return None; };
                Some((x, y, d))
            }).collect();

            // Loop will be skipped if position was surrounded by the same value.
            for &border in &current_borders {
                border_set.insert((x, y, border));

                match border {
                    d @ Direction::Up | d @ Direction::Down => {
                        let mut edge = 1;
                        for &(neighbour_x, neighbour_y, _) in &actual_neighbours {
                            if y == neighbour_y && border_set.contains(&(neighbour_x, neighbour_y, d)) { edge -= 1; }
                        }
                        sized_borders += edge;
                    },
                    d @ Direction::Left | d @ Direction::Right => {
                        let mut edge = 1;
                        for &(neighbour_x, neighbour_y, _) in &actual_neighbours {
                            if x == neighbour_x && border_set.contains(&(neighbour_x, neighbour_y, d)) { edge -= 1; }
                        }
                        sized_borders += edge;
                    },
                }
            }

            borders += current_borders.len() as i32;
            explored.insert((x, y));
        }

        total_score += borders * area;
        bulk_score += sized_borders * area;
    });

    println!("Problem 1: {}", total_score);
    println!("Problem 2: {}", bulk_score);
}
