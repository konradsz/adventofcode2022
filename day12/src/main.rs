use std::collections::{HashMap, VecDeque};

fn part_1(map: &[Vec<u8>]) -> i32 {
    let start = find_on_map(map, b'S')[0];
    let end = find_on_map(map, b'E')[0];
    traverse(map, start, end).unwrap()
}

fn part_2(map: &[Vec<u8>]) -> i32 {
    let mut starting_positions = find_on_map(map, b'S');
    starting_positions.append(&mut find_on_map(map, b'a'));
    let end = find_on_map(map, b'E')[0];

    starting_positions
        .into_iter()
        .filter_map(|start| traverse(map, start, end))
        .min()
        .unwrap()
}

fn find_on_map(map: &[Vec<u8>], needle: u8) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if c == &needle {
                result.push((x, y));
            }
        }
    }

    result
}

struct State {
    position: (usize, usize),
    steps_taken: i32,
}

fn traverse(map: &[Vec<u8>], start: (usize, usize), end: (usize, usize)) -> Option<i32> {
    let height = map.len();
    let width = map[0].len();

    let mut visited: HashMap<(usize, usize), i32> = HashMap::new();
    visited.insert(start, 0);

    let mut to_visit = VecDeque::new();
    to_visit.push_back(State {
        position: start,
        steps_taken: 0,
    });

    while let Some(state) = to_visit.pop_front() {
        if state.position == end {
            break;
        }

        for neighbour in get_neighbours(state.position, height, width) {
            if can_climb(
                map[state.position.1][state.position.0],
                map[neighbour.1][neighbour.0],
            ) {
                if *visited.get(&neighbour).unwrap_or(&i32::MAX) <= state.steps_taken + 1 {
                    continue;
                }

                visited.insert(neighbour, state.steps_taken + 1);

                to_visit.push_back(State {
                    position: neighbour,
                    steps_taken: state.steps_taken + 1,
                });
            }
        }
    }

    visited.get(&end).copied()
}

fn can_climb(from: u8, to: u8) -> bool {
    let from = if from == b'S' { b'a' } else { from } as i8;
    let to = if to == b'E' { b'z' } else { to } as i8;

    to - from <= 1
}

fn get_neighbours(
    position: (usize, usize),
    height: usize,
    width: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let directions = [
        (0, -1), // up
        (1, 0),  // left
        (0, 1),  // down
        (-1, 0), //right
    ];

    let in_bounds = move |position: (i32, i32)| {
        position.0 >= 0
            && position.0 < width as i32
            && position.1 >= 0
            && position.1 < height as i32
    };

    directions.into_iter().filter_map(move |dir| {
        let new_pos = (position.0 as i32 + dir.0, position.1 as i32 + dir.1);
        in_bounds(new_pos).then_some((new_pos.0 as usize, new_pos.1 as usize))
    })
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let map = input
        .lines()
        .map(|l| l.chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    assert_eq!(part_1(&map), 490);
    assert_eq!(part_2(&map), 488);
}
