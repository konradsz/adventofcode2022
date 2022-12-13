use std::collections::HashMap;

fn can_climb(from: char, to: char) -> bool {
    let from = if from == 'S' { 'a' } else { from };
    let to = if to == 'E' { 'z' } else { to };
    let from = from as u8;
    let to = to as u8;
    if to < from {
        true
    } else {
        let elevation = to as u8 - from as u8;
        elevation == 0 || elevation == 1
    }
}

fn traverse(
    map: &Vec<Vec<char>>,
    current_position: (i32, i32),
    current_step: i32,
    end: (i32, i32),
    visited: &mut HashMap<(i32, i32), i32>,
) {
    if let Some(v) = visited.get(&current_position) {
        if v <= &current_step {
            return;
        }
    }

    visited.insert(current_position, current_step);

    if current_position == end {
        return;
    }

    // visited.insert(current_position, current_step);

    let directions = [
        (0, -1), // up
        (1, 0),  // left
        (0, 1),  // down
        (-1, 0), //right
    ];

    for dir in directions {
        let new_pos = (current_position.0 + dir.0, current_position.1 + dir.1);
        if new_pos.0 < 0
            || new_pos.0 >= map[0].len() as i32
            || new_pos.1 < 0
            || new_pos.1 >= map.len() as i32
        {
            continue;
        }
        if can_climb(
            map[current_position.1 as usize][current_position.0 as usize],
            map[new_pos.1 as usize][new_pos.0 as usize],
        ) {
            traverse(map, new_pos, current_step + 1, end, visited);
        }
    }
}
fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut starting_position = Vec::new();
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &'S' || c == &'a' {
                starting_position.push((x as i32, y as i32));
            }
        }
    }
    println!("STARTING POSITION SIZE {}", starting_position.len());

    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|c| *c == 'S') {
            start = (x as i32, y as i32);
        }

        if let Some(x) = row.iter().position(|c| *c == 'E') {
            end = (x as i32, y as i32);
        }
    }

    let mut results = vec![];
    for (idx, start) in starting_position.iter().enumerate() {
        let mut visited = HashMap::new();
        traverse(&map, *start, 0, end, &mut visited);

        // println!("{:?}", visited.get(&end));
        if let Some(steps) = visited.get(&end) {
            println!("{idx}: {steps}");
            results.push(*steps);
        }
    }
    let min = *results.iter().min().unwrap();
    println!("{min}");
}
