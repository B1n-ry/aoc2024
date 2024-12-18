pub fn run(file_input: &str) {
    let file_system: Vec<(Option<usize>, u32)> = file_input.chars().enumerate().filter_map(|(i, c)| {
        let id = if i % 2 == 0 { Some(i / 2) } else { None };

        let length = c.to_digit(10).expect("Format wrong! Encountered non-digit");

        if length == 0 { return None;}

        Some((id, length))
    }).collect();

    let p1 = p1(file_system.clone());
    let p2 = p2(file_system.clone());

    println!("Problem 1: {}", p1);
    println!("Problem 2: {}", p2);
}

fn p1(mut file_system: Vec<(Option<usize>, u32)>) -> usize {
    while !file_system.is_empty() {
        let last_index = file_system.len() - 1;  // This index always exists if vector is not empty
        let (Some(last_id), last_size) = file_system[last_index] else {
            file_system.pop();
            continue;
        };

        let Some(first_empty_index) = file_system.iter().position(|(id, size)| id.is_none() && *size > 0) else {
            break;
        };
        let first_empty_size = file_system[first_empty_index].1;
        let capacity = last_size.min(first_empty_size);

        if capacity >= last_size {
            file_system.pop();
        } else {
            file_system[last_index].1 -= capacity;
        }
        if capacity >= first_empty_size {
            file_system[first_empty_index].0 = Some(last_id);
        } else {
            file_system[first_empty_index].1 -= capacity;
            file_system.insert(first_empty_index, (Some(last_id), capacity));
        }
    }

    let mut total = 0;
    for i in 0.. {
        let Some((Some(first_id), first_size)) = file_system.first_mut() else { break; };
        total += *first_id * i;
        *first_size -= 1;

        if *first_size == 0 {
            file_system.remove(0);
        }
    }

    total
}

fn p2(mut file_system: Vec<(Option<usize>, u32)>) -> usize {
    let mut start_offset = 0;

    while file_system.len() > start_offset {
        let last_index = file_system.len() - start_offset - 1;
        let (Some(last_id), last_size) = file_system[last_index] else {
            start_offset += 1;
            continue;
        };

        let Some(empty_space_index) = file_system.iter().position(|(id, size)| id.is_none() && *size >= last_size) else {
            start_offset += 1;
            continue;
        };

        if empty_space_index >= last_index {
            start_offset += 1;
            continue;
        }

        file_system[last_index].0 = None;
        let (empty_space_id, empty_space_size) = &mut file_system[empty_space_index];
        if *empty_space_size == last_size {
            *empty_space_id = Some(last_id);
        } else {
            *empty_space_size -= last_size;
            file_system.insert(empty_space_index, (Some(last_id), last_size));
        }
    }
    
    let mut total = 0;
    for i in 0.. {
        let Some((first_id, first_size)) = file_system.first_mut() else { break; };
        
        first_id.inspect(|&id| total += id * i);
        *first_size -= 1;

        if *first_size == 0 {
            file_system.remove(0);
        }
    }

    total
}
