use std::collections::HashMap;

struct Cube {
    x: u32,
    y: u32,
    z: u32,
    sides_exposed: [bool; 6],
}

impl Cube {
    fn new(x: u32, y: u32, z: u32) -> Self {
        Self {
            x,
            y,
            z,
            sides_exposed: [true; 6],
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let sides = HashMap::from([
        (0, (0, 0, 1)),
        (1, (-1, 0, 0)),
        (2, (0, 1, 0)),
        (3, (0, 0, -1)),
        (4, (1, 0, 0)),
        (5, (0, -1, 0)),
    ]);

    // let adjacent_sides = [
    //     ((1, 0, 0), (-1, 0, 0)),
    //     ((-1, 0, 0), (1, 0, 0)),
    //     ((0, 1, 0), (0, -1, 0)),
    //     ((0, -1, 0), (0, 1, 0)),
    //     ((0, 0, 1), (0, 0, -1)),
    //     ((0, 0, -1), (0, 0, 1)),
    // ];

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
        .collect::<Vec<_>>();

    let mut cubes = input
        .lines()
        .map(|l| {
            let mut coords = l.split(',').map(|coord| coord.parse::<i32>().unwrap());
            (
                (
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                ),
                [true; 6],
            )
        })
        .collect::<HashMap<_, _>>();

    for cube in coords {
        for (idx, side) in sides.iter() {
            let neighbour_coords = (cube.0 + side.0, cube.1 + side.1, cube.2 + side.2);
            if let Some(cube) = cubes.get_mut(&neighbour_coords) {
                cube[(idx + 3) % 6] = false;
            } else {
                continue;
            }
            let c = cubes.get_mut(&cube).unwrap();
            c[*idx] = false;
        }
    }

    let sum = cubes
        .values()
        .map(|exposed| exposed.iter().filter(|e| **e).count())
        .sum::<usize>();
    println!("{sum}");
}
