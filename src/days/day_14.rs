const ROOM_WIDTH: i32 = 101;
const ROOM_HEIGHT: i32 = 103;

pub fn run(file_input: &str) {
    let mut robots: Vec<(i32, i32, i32, i32)> = file_input.lines().map(|line| {
        let s: String = line.chars().map(|c| if c.is_ascii_digit() || c == '-' { c } else { ' ' }).collect();
        let numbers: Vec<i32> = s.split_ascii_whitespace().map(|digit| digit.parse().expect("Expected digit")).collect();

        assert_eq!(numbers.len(), 4);
        (numbers[0], numbers[1], numbers[2], numbers[3])
    }).collect();

    for i in 0.. {
        if i == 100 {
            let safety_value = get_safety(&robots);
            println!("Problem 1: {}", safety_value);
        }
        if is_portraying_christmas_tree(&robots) {
            println!("Problem 2: {}", i);
            break;
        }
        move_all_robots(&mut robots);
    }
}

fn move_all_robots(robots: &mut Vec<(i32, i32, i32, i32)>) {
    robots.iter_mut().for_each(|(p_x, p_y, v_x, v_y)| {
        *p_x = (*p_x + *v_x) % ROOM_WIDTH;
        if *p_x < 0 { *p_x += ROOM_WIDTH }

        *p_y = (*p_y + *v_y) % ROOM_HEIGHT;
        if *p_y < 0 { *p_y += ROOM_HEIGHT }
    });
}
fn get_safety(robots: &Vec<(i32, i32, i32, i32)>) -> i32 {
    let mut quadrants = (0, 0, 0, 0);
    for &(p_x, p_y, _, _) in robots {
        if p_x < ROOM_WIDTH / 2 && p_y < ROOM_HEIGHT / 2 {
            quadrants.0 += 1;
        }
        else if p_x > ROOM_WIDTH / 2 && p_y < ROOM_HEIGHT / 2 {
            quadrants.1 += 1;
        }
        else if p_x < ROOM_WIDTH / 2 && p_y > ROOM_HEIGHT / 2 {
            quadrants.2 += 1;
        }
        else if p_x > ROOM_WIDTH / 2 && p_y > ROOM_HEIGHT / 2 {
            quadrants.3 += 1;
        }
    }

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

fn is_portraying_christmas_tree(robots: &Vec<(i32, i32, i32, i32)>) -> bool {
    let mut room = [[0; ROOM_WIDTH as usize]; ROOM_HEIGHT as usize];

    for &(p_x, p_y, _, _) in robots {
        room[p_y as usize][p_x as usize] += 1;
    }

    // Finding 8 in a row seems to be good enough to assume this iteration is the christmas tree
    room.iter().any(|row| row.windows(8).any(|slice| !slice.contains(&0)))
}
