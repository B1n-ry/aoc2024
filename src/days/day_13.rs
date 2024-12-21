use std::isize;

const A_TOKENS: isize = 3;
const B_TOKENS: isize = 1;

pub fn run(file_input: &str) {
    let games: Vec<_> = file_input.split("\r\n\r\n").map(|game| {
        let game_lines: Vec<&str> = game.lines().collect();
        assert_eq!(game_lines.len(), 3);
        (
            get_from_game_line(game_lines[0], "Button A", "+"),
            get_from_game_line(game_lines[1], "Button B", "+"),
            get_from_game_line(game_lines[2], "Prize", "="),
        )
    }).collect();

    let tokens: isize = get_tokens(&games, 0);
    let tokens_increased: isize = get_tokens(&games, 10000000000000);

    println!("Problem 1: {}", tokens);
    println!("Problem 2: {}", tokens_increased);
}

fn get_from_game_line(game_line: &str, line_start: &str, separator: &str) -> (isize, isize) {
    let (_, rest_of_string) = game_line.split_once(&format!("{}: X{}", line_start, separator)).expect("Wrong format!");
    let (x_string, y_string) = rest_of_string.split_once(&format!(", Y{}", separator)).expect("Wrong format!");
    (x_string.parse().expect("X not a number"), y_string.parse().expect("Y not a number"))
}

fn get_tokens(games: &Vec<((isize, isize), (isize, isize), (isize, isize))>, prize_offset: isize) -> isize {
    games.iter().map(|game| {
        let &((a_x_diff, a_y_diff), (b_x_diff, b_y_diff), (prize_x, prize_y)) = game;
        let (prize_x, prize_y) = (prize_x + prize_offset, prize_y + prize_offset);

        let det = a_x_diff * b_y_diff - a_y_diff * b_x_diff;

        if det == 0 {  // We move in a straight line
            let mut tokens: Option<isize> = None;
            if prize_x % a_x_diff == 0 && prize_y % a_y_diff == 0 {
                if prize_x / a_x_diff == prize_y / a_y_diff {
                    let local_tokens = (prize_x / a_x_diff) * A_TOKENS;
                    tokens = Some(tokens.map_or(local_tokens, |prev_tokens| prev_tokens.min(local_tokens)));
                }
            }
            if prize_x % b_x_diff == 0 && prize_y % b_x_diff == 0 {
                if prize_x / b_x_diff == prize_y / b_y_diff {
                    let local_tokens = (prize_x / b_x_diff) * B_TOKENS;
                    tokens = Some(tokens.map_or(local_tokens, |prev_tokens| prev_tokens.min(local_tokens)));
                }
            }
            return tokens.unwrap_or(0);
        }

        let n = (prize_x * b_y_diff - prize_y * b_x_diff) as f64 / det as f64;
        let m = (prize_y * a_x_diff - prize_x * a_y_diff) as f64 / det as f64;
        if n > 0.0 && m > 0.0 && n.fract() == 0.0 && m.fract() == 0.0 {
            n as isize * A_TOKENS + m as isize * B_TOKENS
        } else {
            0
        }
    }).sum()
}
