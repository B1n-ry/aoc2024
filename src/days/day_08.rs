use std::{collections::{HashMap, HashSet}, fmt::Display, ops::Mul};

struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn from(double_vec: Vec<Vec<char>>) -> Self {
        let width = double_vec.get(0).map_or(0, |v| v.len());
        let height = double_vec.len();

        Self { grid: double_vec, width, height }
    }

    fn get(&self, x: usize, y: usize) -> Option<&char> {
        self.grid.get(y).and_then(|v| v.get(x))
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut char> {
        self.grid.get_mut(y).and_then(|v| v.get_mut(x))
    }
    fn set(&mut self, x: usize, y: usize, c: char) {
        let Some(old) = self.get_mut(x, y) else { return; };
        *old = c;
    }
    fn is_within_bounds(&self, x: usize, y: usize) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    fn for_each(&self, mut func: impl FnMut(usize, usize, char)) {
        for y in 0..self.height {
            for x in 0..self.width {
                let Some(&char) = self.get(x, y) else { continue; };
                func(x, y, char);
            }
        }
    }
}

pub fn run(file_input: &str) {
    let mut antenna_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let double_vec = file_input.lines().map(|line| line.chars().collect()).collect();
    let mut grid = Grid::from(double_vec);  // Moved value

    grid.for_each(|x, y, c| {
        if c != '.' {
            match antenna_locations.get_mut(&c) {
                Some(v) => { v.push((x, y)); },
                None => { antenna_locations.insert(c, vec![(x, y)]); },
            };
        }
    });

    let antinodes: HashSet<(usize, usize)> = antenna_locations.values().flat_map(|v| {
        let mut antinodes = Vec::new();
        for (i, &(x1, y1)) in v.iter().enumerate() {
            for &(x2, y2) in v.iter().skip(i + 1) {
                let (x_diff, y_diff) = (x1.abs_diff(x2), y1.abs_diff(y2));
                for n in 0.. {
                    let (x_diff, y_diff) = (x_diff.mul(n), y_diff.mul(n));
                    let mut local_antinodes = if x1 <= x2 && y1 <= y2 {
                        vec![(x1.wrapping_sub(x_diff), y1.wrapping_sub(y_diff)), (x2.wrapping_add(x_diff), y2.wrapping_add(y_diff))]
                    } else if x1 >= x2 && y1 >= y2 {
                        vec![(x2.wrapping_sub(x_diff), y2.wrapping_sub(y_diff)), (x1.wrapping_add(x_diff), y1.wrapping_add(y_diff))]
                    } else if x1 <= x2 && y1 >= y2 {
                        vec![(x1.wrapping_sub(x_diff), y1.wrapping_add(y_diff)), (x2.wrapping_add(x_diff), y2.wrapping_sub(y_diff))]
                    } else if x1 >= x2 && y1 <= y2 {
                        vec![(x2.wrapping_sub(x_diff), y2.wrapping_add(y_diff)), (x1.wrapping_add(x_diff), y1.wrapping_sub(y_diff))]
                    } else {
                        panic!("Can't have two antennas at the same place!");
                    };
                    for j in (0..local_antinodes.len()).rev() {
                        let (x, y) = local_antinodes[j];
                        if !grid.is_within_bounds(x, y) {
                            local_antinodes.remove(j);
                        }
                    }
                    if local_antinodes.is_empty() {
                        break;
                    }
                    antinodes.append(&mut local_antinodes);
                }
            }
        }
        antinodes
    }).collect();

    println!("Problem 1: {}", antinodes.len());
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid.iter().map(|row| row.iter().map(|value| format!("{}", value)).collect::<String>()).collect::<Vec<String>>().join("\n"))
    }
}
