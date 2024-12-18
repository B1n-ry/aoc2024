use std::{collections::{HashMap, HashSet}, str::Chars};

#[allow(dead_code)]
struct Grid<T> {
    grid: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl Grid<char> {
    fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.grid.get(y).map_or(None, |v| v.get(x))
    }

    fn get_all(&self, predicate: fn(c: &char) -> bool) -> Vec<(usize, usize)> {
        self.grid.iter().enumerate().flat_map(|(y, row)| row.iter().enumerate().filter_map(move |(x, col)| {
            if predicate(col) { Some((x, y)) } else { None }
        })).collect()
    }
}
impl<'a> FromIterator<Chars<'a>> for Grid<char> {
    fn from_iter<T: IntoIterator<Item = Chars<'a>>>(iter: T) -> Self {
        let grid: Vec<Vec<char>> = iter.into_iter().map(|i| i.collect()).collect();
        let height = grid.len();
        let width = grid.get(0).map_or(0, |row| row.len());

        Self { grid, width, height }
    }
}

pub fn run(file_input: &str) {
    let grid: Grid<char> = file_input.lines().map(|line| line.chars()).collect();
    let starts = grid.get_all(|&c| c == '0');

    let following_chars = HashMap::from([
        ('0', '1'),
        ('1', '2'),
        ('2', '3'),
        ('3', '4'),
        ('4', '5'),
        ('5', '6'),
        ('6', '7'),
        ('7', '8'),
        ('8', '9'),
    ]);

    let mut total_ends = 0;
    let mut total_rating = 0;

    for (x, y) in starts {
        let mut final_positions = Vec::new();
        let mut exploration_queue = vec![(x, y)];

        while let Some((x, y)) = exploration_queue.pop() {
            let &current_char = grid.get(x, y).expect("Position should be defined in grid, but isn't");
            if current_char == '9' {
                final_positions.push((x, y));
                continue;
            }

            let next_char = following_chars.get(&current_char).expect("Format wrong! Detected something not between 0 and 9 in grid!");
            for (next_x, next_y) in [
                (x.saturating_add(1), y),
                (x.saturating_sub(1), y),
                (x, y.saturating_add(1)),
                (x, y.saturating_sub(1)),
            ] {
                if grid.get(next_x, next_y) == Some(next_char) {
                    exploration_queue.push((next_x, next_y));
                }
            }
        }

        total_rating += final_positions.len();

        let set: HashSet<(usize, usize)> = HashSet::from_iter(final_positions);
        total_ends += set.len();
    }

    println!("Problem 1: {}", total_ends);
    println!("Problem 2: {}", total_rating);
}
