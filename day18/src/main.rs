use std::collections::{HashMap, HashSet, VecDeque};

const SIDES: [(i32, i32, i32); 6] = [
    (0, 0, 1),
    (-1, 0, 0),
    (0, 1, 0),
    (0, 0, -1),
    (1, 0, 0),
    (0, -1, 0),
];

fn count_exposed(coords: &HashSet<(i32, i32, i32)>) -> usize {
    let mut cubes = coords
        .iter()
        .map(|coords| (coords, [true; 6]))
        .collect::<HashMap<_, _>>();

    for cube in coords {
        for (idx, side) in SIDES.iter().enumerate() {
            let neighbour_coords = (cube.0 + side.0, cube.1 + side.1, cube.2 + side.2);
            if let Some(cube) = cubes.get_mut(&neighbour_coords) {
                cube[(idx + 3) % 6] = false;
            } else {
                continue;
            }
            cubes.get_mut(cube).unwrap()[idx] = false;
        }
    }

    cubes
        .values()
        .map(|exposed| exposed.iter().filter(|e| **e).count())
        .sum()
}

fn remove_air_pockets(cubes: &HashSet<(i32, i32, i32)>) -> HashSet<(i32, i32, i32)> {
    let (max_x, max_y, max_z) = cubes.iter().fold(
        (i32::MIN, i32::MIN, i32::MIN),
        |(max_x, max_y, max_z), coords| {
            (
                std::cmp::max(max_x, coords.0),
                std::cmp::max(max_y, coords.1),
                std::cmp::max(max_z, coords.2),
            )
        },
    );

    let mut to_check = VecDeque::new();
    to_check.push_back((0, 0, 0));
    let mut water = HashSet::new();

    while let Some(c) = to_check.pop_front() {
        if c.0 < 0 || c.0 > max_x || c.1 < 0 || c.1 > max_y || c.2 < 0 || c.2 > max_z {
            continue;
        }
        if cubes.contains(&c) {
            continue;
        } else if water.insert(c) {
            SIDES.iter().for_each(|s| {
                let new_coords = (c.0 + s.0, c.1 + s.1, c.2 + s.2);
                to_check.push_back(new_coords);
            });
        }
    }

    let mut entire_space = HashSet::new();
    for x in 0..=max_x {
        for y in 0..=max_y {
            for z in 0..=max_z {
                entire_space.insert((x, y, z));
            }
        }
    }

    entire_space.difference(&water).copied().collect()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let coords = input
        .lines()
        .map(|l| {
            let mut coords = l.split(',').map(|coord| coord.parse::<i32>().unwrap());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect::<HashSet<_>>();

    let part_1 = count_exposed(&coords);
    assert_eq!(part_1, 4456);

    let coords_no_air_pockets = remove_air_pockets(&coords);
    let part_2 = count_exposed(&coords_no_air_pockets);
    assert_eq!(part_2, 2510);
}
