use std::collections::{HashMap, VecDeque};

fn part_1(map: &[Vec<char>]) -> i32 {
    let start = find_on_map(map, 'S')[0];
    let end = find_on_map(map, 'E')[0];
    traverse(&map, start, end).unwrap()
}

fn part_2(map: &[Vec<char>]) -> i32 {
    let mut starting_positions = find_on_map(map, 'S');
    starting_positions.append(&mut find_on_map(map, 'a'));
    let end = find_on_map(map, 'E')[0];

    let mut best = i32::MAX;
    for start in starting_positions {
        if let Some(steps) = traverse(&map, start, end) {
            if steps < best {
                best = steps;
            }
        }
    }
    best
}

fn find_on_map(map: &[Vec<char>], needle: char) -> Vec<(i32, i32)> {
    let mut result = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &needle {
                result.push((x as i32, y as i32));
            }
        }
    }

    result
}

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

struct State {
    position: (i32, i32),
    steps_taken: i32,
}

fn traverse(map: &[Vec<char>], start: (i32, i32), end: (i32, i32)) -> Option<i32> {
    let mut to_visit = VecDeque::new();
    to_visit.push_back(State {
        position: start,
        steps_taken: 0,
    });

    let directions = [
        (0, -1), // up
        (1, 0),  // left
        (0, 1),  // down
        (-1, 0), //right
    ];

    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();
    visited.insert(start, 0);

    while let Some(state) = to_visit.pop_front() {
        if state.position == end {
            break;
        }

        for dir in directions {
            let new_pos = (state.position.0 + dir.0, state.position.1 + dir.1);
            if new_pos.0 < 0
                || new_pos.0 >= map[0].len() as i32
                || new_pos.1 < 0
                || new_pos.1 >= map.len() as i32
            {
                continue;
            }

            if can_climb(
                map[state.position.1 as usize][state.position.0 as usize],
                map[new_pos.1 as usize][new_pos.0 as usize],
            ) {
                if *visited.get(&new_pos).unwrap_or(&i32::MAX) <= state.steps_taken + 1 {
                    continue;
                }

                visited.insert(new_pos, state.steps_taken + 1);

                to_visit.push_back(State {
                    position: new_pos,
                    steps_taken: state.steps_taken + 1,
                });
            }
        }
    }

    visited.get(&end).copied()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    assert_eq!(part_1(&map), 490);
    assert_eq!(part_2(&map), 488);
}
