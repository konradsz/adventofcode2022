use std::collections::HashSet;

fn get_shape(turn: u64) -> Vec<(u32, u32)> {
    match turn % 5 {
        0 => vec![(2, 3), (3, 3), (4, 3), (5, 3)],
        1 => vec![(2, 4), (3, 5), (3, 4), (3, 3), (4, 4)],
        2 => vec![(2, 3), (3, 3), (4, 5), (4, 4), (4, 3)],
        3 => vec![(2, 6), (2, 5), (2, 4), (2, 3)],
        4 => vec![(2, 4), (3, 4), (2, 3), (3, 3)],
        _ => unreachable!(),
    }
}

fn try_moving_left(room: &HashSet<(u32, u32)>, shape: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    if shape.iter().any(|(x, _)| x == &0) {
        return shape;
    }

    let moved = shape.iter().map(|(x, y)| (x - 1, *y)).collect::<Vec<_>>();
    if moved.iter().all(|el| room.get(el).is_none()) {
        moved
    } else {
        shape
    }
}

fn try_moving_right(room: &HashSet<(u32, u32)>, shape: Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    if shape.iter().any(|(x, _)| x == &6) {
        return shape;
    }

    let moved = shape.iter().map(|(x, y)| (x + 1, *y)).collect::<Vec<_>>();
    if moved.iter().all(|el| room.get(el).is_none()) {
        moved
    } else {
        shape
    }
}

fn try_moving_down(room: &HashSet<(u32, u32)>, shape: &mut [(u32, u32)]) -> bool {
    if shape.iter().any(|(_, y)| y == &0) {
        return false;
    }

    if shape.iter().all(|(x, y)| room.get(&(*x, y - 1)).is_none()) {
        shape.iter_mut().for_each(|(_, y)| *y -= 1);
        true
    } else {
        false
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let directions = input.chars().collect::<Vec<_>>();

    let mut room = HashSet::new();
    let mut heighest_point = 0;
    let mut current_dir = 0;
    for turn in 0..2022 {
        let mut shape = get_shape(turn);
        for el in shape.iter_mut() {
            let offset = if turn == 0 { 0 } else { 1 };
            el.1 += heighest_point + offset;
        }

        loop {
            match directions[current_dir % directions.len()] {
                '<' => shape = try_moving_left(&room, shape),
                '>' => shape = try_moving_right(&room, shape),
                _ => panic!("unknown direction"),
            };
            current_dir += 1;
            if !try_moving_down(&room, &mut shape) {
                shape.into_iter().for_each(|el| assert!(room.insert(el)));
                break;
            }
        }

        heighest_point = room.iter().map(|el| el.1).max().unwrap();
    }

    let part_1 = room.iter().map(|(_, y)| *y).max().unwrap() + 1;
    assert_eq!(part_1, 3193);

    // PART 2:
    // cycle starts after 1929 turns with tower height 3045
    // cycle size is 1745 changing height by 2753
    // (1000000000000-1929) / 1745 = 573065901
    // (1000000000000-1929) % 1745 = 826
    // after 1929 + 826 turns height equals 4382
    // 4382 - 3045 = 1337
    // (1000000000000-1929)/1745 * 2753 + 3045 + 1337 = 1577650429835
}
