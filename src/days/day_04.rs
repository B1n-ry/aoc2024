use std::char;

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    DownLeft,
    DownRight,
    UpLeft,
    UpRight,
}

// Look I know this is not a good way to implement traits, as the return type is statically defined.
// I just wanted to call from the tuple on the step() function, and Option<Self> had unknown size to the compiler.
trait Coordinated {
    fn step(&self, direction: &Direction, steps: usize) -> Option<(usize, usize)>;
}
impl Coordinated for (usize, usize) {
    fn step(&self, direction: &Direction, steps: usize) -> Option<(usize, usize)> {
        match direction {
            Direction::Up => self.1.checked_sub(steps).map(|y| (self.0, y)),
            Direction::Down => self.1.checked_add(steps).map(|y| (self.0, y)),
            Direction::Left => self.0.checked_sub(steps).map(|x| (x, self.1)),
            Direction::Right => self.0.checked_add(steps).map(|x| (x, self.1)),
            Direction::UpLeft => self.0.checked_sub(steps).zip(self.1.checked_sub(steps)),
            Direction::UpRight => self.0.checked_add(steps).zip(self.1.checked_sub(steps)),
            Direction::DownLeft => self.0.checked_sub(steps).zip(self.1.checked_add(steps)),
            Direction::DownRight => self.0.checked_add(steps).zip(self.1.checked_add(steps)),
        }
    }
}

impl Direction {
    fn values() -> [Self; 8] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::DownLeft,
            Direction::DownRight,
            Direction::UpLeft,
            Direction::UpRight,
        ]
    }
}

pub fn run(file_input: &str) {
    let char_map = file_input.lines().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    assert!(char_map.len() > 0 && char_map[0].len() > 0);

    let (height, width) = (char_map.len(), char_map[0].len());

    let mut xmas_count = 0;
    let mut x_mas_count = 0;

    for row in 0..height {  // Vertical
        for col in 0..width {
            if char_map[row][col] != 'X' { continue; }
            let matches: i32 = Direction::values().iter().map(|dir| recursive_word_search(dir, "XMAS", (col, row), &char_map) as i32).sum();
            xmas_count += matches;
        }
    }

    for row in 0..height {
        for col in 0..width {
            if char_map[row][col] != 'A' { continue; }
            if row < 1 || row >= height - 1 || col < 1 || col >= width - 1 { continue; }
            let downright_upleft = [
                char_map[row + 1][col + 1],
                char_map[row - 1][col - 1],
            ];
            let downleft_upright = [
                char_map[row + 1][col - 1],
                char_map[row - 1][col + 1],
            ];
            if downleft_upright.contains(&'M') && downleft_upright.contains(&'S') && downright_upleft.contains(&'M') && downright_upleft.contains(&'S') {
                x_mas_count += 1;
            }
        }
    }

    println!("Problem 1: {}", xmas_count);
    println!("Problem 2: {}", x_mas_count);
}

fn recursive_word_search(direction: &Direction, searched: &str, search_from: (usize, usize), search_in: &Vec<Vec<char>>) -> bool {
    if searched.is_empty() { return true; }

    let (height, width) = (search_in.len(), search_in.get(0).map_or(0, |v| v.len()));
    let (x, y) = search_from;

    if !(0..width).contains(&x) || !(0..height).contains(&y) {
        return false;
    }

    if !searched.starts_with(search_in[y][x]) { return false; }
    
    let Some(new_pos) = search_from.step(&direction, 1) else { return searched.len() == 1; };
    recursive_word_search(direction, &searched[1..], new_pos, search_in)
}
