use std::{char, fmt::Write, ops::Mul};

pub fn run(file_input: &str) {
    let char_map = file_input.lines().map(|line| {
        line.chars().collect::<Vec<char>>()
    }).collect::<Vec<Vec<char>>>();

    assert!(char_map.len() > 0 && char_map[0].len() > 0);

    let (height, width) = (char_map.len() as isize, char_map[0].len() as isize);

    let mut xmas_count = 0;
    let mut x_mas_count = 0;

    let mut word_buffer = String::new();
    for row in 0..height {  // Vertical
        word_buffer.clear();
        for col in 0..width {
            if is_xmas_with_char(&mut word_buffer, char_map[row as usize][col as usize]) {
                xmas_count += 1;
            }
        }
    }
    for col in 0..width {  // Horizontal
        word_buffer.clear();
        for row in 0..height {
            if is_xmas_with_char(&mut word_buffer, char_map[row as usize][col as usize]) {
                xmas_count += 1;
            }
        }
    }
    for row in -width..height {  // Diagonal up-left to down-right
        word_buffer.clear();
        for col in 0..width {
            let row_pos = row + col;
            if row_pos < 0 { continue; }
            if row_pos >= height { continue; }
            if is_xmas_with_char(&mut word_buffer, char_map[row_pos as usize][col as usize]) {
                xmas_count += 1;
            }
        }
    }
    for row in 0..height.mul(2) {  // Diagonal down-left to up-right
        word_buffer.clear();
        for col in 0..width {
            let row_pos = row - col;
            if row_pos < 0 { continue; }
            if row_pos >= height { continue; }
            if is_xmas_with_char(&mut word_buffer, char_map[row_pos as usize][col as usize]) {
                xmas_count += 1;
            }
        }
    }

    for row in 0..height {
        for col in 0..width {
            if char_map[row as usize][col as usize] != 'A' { continue; }
            if row < 1 || row >= height - 1 || col < 1 || col >= width - 1 { continue; }
            let downright_upleft = [
                char_map[(row + 1) as usize][(col + 1) as usize],
                char_map[(row - 1) as usize][(col - 1) as usize],
            ];
            let downleft_upright = [
                char_map[(row + 1) as usize][(col - 1) as usize],
                char_map[(row - 1) as usize][(col + 1) as usize],
            ];
            if downleft_upright.contains(&'M') && downleft_upright.contains(&'S') && downright_upleft.contains(&'M') && downright_upleft.contains(&'S') {
                x_mas_count += 1;
            }
        }
    }

    println!("Problem 1: {}", xmas_count);
    println!("Problem 2: {}", x_mas_count);
}

fn is_xmas_with_char(buffer: &mut String, c: char) -> bool {
    buffer.write_char(c).expect("Write failed");
    if buffer.len() > 4 {
        buffer.remove(0);
    }
    buffer == "XMAS" || buffer == "SAMX"
}
