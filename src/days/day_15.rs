use std::{fmt::Display, ops::Add, str::FromStr};

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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}
impl Direction {
    fn is_horizontal(&self) -> bool {
        *self == Direction::Left || *self == Direction::Right
    }
}
impl FromStr for Direction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "v" => Ok(Direction::Down),
            "<" => Ok(Direction::Left),
            ">" => Ok(Direction::Right),
            "^" => Ok(Direction::Up),
            _ => Err(String::from("Could not parse")),
        }
    }
}
impl From<char> for Direction {
    fn from(s: char) -> Self {
        match s {
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            '^' => Direction::Up,
            _ => panic!("Could not parse char: {}", s),
        }
    }
}

impl Add<Direction> for (usize, usize) {
    type Output = (usize, usize);
    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Down => (self.0, self.1 + 1),
            Direction::Left => (self.0 - 1, self.1),
            Direction::Right => (self.0 + 1, self.1),
            Direction::Up => (self.0, self.1 - 1),
        }
    }
}

pub fn run(file_input: &str) {
    let (warehouse, instructions) = file_input.split_once("\r\n\r\n").expect("Format wrong! No gap found!");

    let mut grid: Grid<char> = warehouse.lines().map(|line| line.chars().collect()).collect();
    let mut bigger_warehouse: Grid<char> = warehouse.lines().map(|line| line.chars().flat_map(|c| {
        match c {
            'O' => ['[', ']'],
            '@' => ['@', '.'],
            a @ _ => [a, a],
        }
    }).collect()).collect();
    let robot = grid.get_all(|c| c == &'@');
    assert_eq!(robot.len(), 1);
    let mut robot = robot[0];

    let robot_2 = bigger_warehouse.get_all(|c| c == &'@');
    assert_eq!(robot_2.len(), 1);
    let mut robot_2 = robot_2[0];

    instructions.chars().for_each(|c| {
        if !['<', '>', 'v', '^'].contains(&c) { return; }  // Newline characters that are supposed to be ignored

        assert_eq!(bigger_warehouse.get(robot_2.0, robot_2.1), Some(&'@'));

        let direction = Direction::from(c);
        if try_move(robot, &mut grid, &direction) {
            robot = robot + direction;
        }
        if can_move(robot_2, &bigger_warehouse, &direction) {
            try_move(robot_2, &mut bigger_warehouse, &direction);
            robot_2 = robot_2 + direction;
        }
    });

    let mut gps_sum = 0;
    grid.for_each(|x, y, &c| {
        if c == 'O' {
            gps_sum += 100 * y + x;
        }
    });
    let mut gps_sum_2 = 0;
    bigger_warehouse.for_each(|x, y, &c| {
        if c == '[' {
            gps_sum_2 += 100 * y + x;
        }
    });

    println!("Problem 1: {}", gps_sum);
    println!("Problem 2: {}", gps_sum_2);
}

fn can_move((current_x, current_y): (usize, usize), grid: &Grid<char>, direction: &Direction) -> bool {
    let (x, y) = (current_x, current_y) + *direction;
    match grid.get(x, y) {
        Some(&'#') => false,
        Some(&'O') => can_move((x, y), grid, direction),
        Some(&'.') => true,
        Some(&'[') => can_move((x, y), grid, direction) && (direction.is_horizontal() || can_move((x, y) + Direction::Right, grid, direction)),
        Some(&']') => can_move((x, y), grid, direction) && (direction.is_horizontal() || can_move((x, y) + Direction::Left, grid, direction)),
        _ => false,
    }
}
fn try_move((current_x, current_y): (usize, usize), grid: &mut Grid<char>, direction: &Direction) -> bool {
    let (x, y) = (current_x, current_y) + *direction;
    let current_char = *grid.get(current_x, current_y).expect("Cell in memory was also not in memory");
    match grid.get(x, y) {
        Some(&'#') => false,
        Some(&'O') => {
            if try_move((x, y), grid, direction) {
                move_cell(grid, (current_x, current_y), direction);
                true
            } else {
                false
            }
        },
        Some(&'.') => {
            move_cell(grid, (current_x, current_y), direction);
            true
        },
        Some(&'[') => {
            // If we are pushing from the same prt of a box, it should push the other side already
            if try_move((x, y), grid, direction) && (direction.is_horizontal() || current_char == '[' || try_move((x, y) + Direction::Right, grid, direction)) {
                move_cell(grid, (current_x, current_y), direction);
                //if !direction.is_horizontal() && current_char != '[' { move_cell(grid, (current_x, current_y) + Direction::Right, direction); }
                true
            } else {
                false
            }
        },
        Some(&']') => {
            // If we are pushing from the same prt of a box, it should push the other side already
            if try_move((x, y), grid, direction) && (direction.is_horizontal() || current_char == ']' || try_move((x, y) + Direction::Left, grid, direction)) {
                move_cell(grid, (current_x, current_y), direction);
                //if !direction.is_horizontal() && current_char != ']' { move_cell(grid, (current_x, current_y) + Direction::Left, direction); }
                true
            } else {
                false
            }
        },
        _ => false,
    }
}

fn move_cell(grid: &mut Grid<char>, (from_x, from_y): (usize, usize), direction: &Direction) {
    let (x, y) = (from_x, from_y) + *direction;
    let moving_value = grid.get(from_x, from_y).expect("Value which should have existed didn't");
    if moving_value == &'.' {
        return;  // Don't move empty cells (as that probably means it's already moved)
    }
    grid.set(x, y, *moving_value);
    grid.set(from_x, from_y, '.');
}
