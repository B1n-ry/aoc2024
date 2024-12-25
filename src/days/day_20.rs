use std::{collections::{BinaryHeap, HashMap}, ops::{Deref, DerefMut}, sync::Mutex};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn run(file_input: &str) {
    let grid = Grid(file_input.lines().map(|line| line.chars().collect()).collect());
    let start = grid.find('S').expect("Map did not include start pos");
    let end = grid.find('E').expect("Map did not include end pos");

    let from_start_costs = dijkstra(start, &grid);
    let from_end_costs = dijkstra(end, &grid);

    let &min_steps_fair = from_start_costs.get(&end).expect("There's no plausible path from start to end");

    let time_save_goal = 100;

    let time_saves_2 = Mutex::new(0);
    let time_saves_20 = Mutex::new(0);
    from_start_costs.par_iter().for_each(|(&(start_x, start_y), &start_c)| {
        from_end_costs.par_iter().for_each(|(&(end_x, end_y), &end_c)| {
            let difference = (start_x.abs_diff(end_x) + start_y.abs_diff(end_y)) as u32;
            if difference <= 2 && start_c + end_c + difference + time_save_goal <= min_steps_fair {
                *time_saves_2.lock().unwrap() += 1;
            }
            if difference <= 20 && start_c + end_c + difference + time_save_goal <= min_steps_fair {
                *time_saves_20.lock().unwrap() += 1;
            }
        });
    });

    println!("Problem 1: {}", time_saves_2.lock().unwrap());
    println!("Problem 2: {}", time_saves_20.lock().unwrap());  // Between 983595 and 1049352
}

struct Grid(Vec<Vec<char>>);
impl Deref for Grid {
    type Target = Vec<Vec<char>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[allow(unused)]
impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<&char> {
        (**self).get(y).map_or(None, |v| v.get(x))
    }
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut char> {
        (**self).get_mut(y).map_or(None, |v| v.get_mut(x))
    }
    fn find(&self, matching: char) -> Option<(usize, usize)> {
        for (y, row) in self.iter().enumerate() {
            for (x, &col) in row.iter().enumerate() {
                if col == matching {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

#[derive(PartialEq, Eq)]
struct DijkstraNode(u32, (usize, usize));
impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.cmp(&self.0)
    }
}
impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(from: (usize, usize), grid: &Grid) -> HashMap<(usize, usize), u32> {
    let mut costs = HashMap::new();

    let mut queue = BinaryHeap::from([DijkstraNode(0, from)]);
    while let Some(DijkstraNode(cost, pos)) = queue.pop() {
        if costs.contains_key(&pos) { continue; }

        costs.insert(pos, cost);
        let (x, y) = pos;

        for (x, y) in [
            (x.checked_add(1), Some(y)),
            (x.checked_sub(1), Some(y)),
            (Some(x), y.checked_add(1)),
            (Some(x), y.checked_sub(1)),
        ] {
            let (Some(x), Some(y)) = (x, y) else { continue; };
            if grid.get(x, y).is_some_and(|&c| c != '#') {
                queue.push(DijkstraNode(cost + 1, (x, y)));
            }
        }
    }

    costs
}
