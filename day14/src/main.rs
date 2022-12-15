use std::{
    cmp::{max, min},
    collections::HashMap,
};

#[derive(Debug, Clone, Copy)]
enum Material {
    Air,
    Rock,
    Sand,
}

fn part_1(mut cave: HashMap<(u32, u32), Material>, max_y: u32) -> u32 {
    for i in 0.. {
        let mut pos = (500, 0);

        loop {
            if pos.1 == max_y {
                return i;
            }
            match cave.get(&(pos.0, pos.1 + 1)).unwrap_or(&Material::Air) {
                Material::Air => pos.1 += 1,
                Material::Rock | Material::Sand => {
                    if cave.get(&(pos.0 - 1, pos.1 + 1)).is_none() {
                        pos.0 -= 1;
                        pos.1 += 1;
                    } else if cave.get(&(pos.0 + 1, pos.1 + 1)).is_none() {
                        pos.0 += 1;
                        pos.1 += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        cave.insert(pos, Material::Sand);
    }

    unreachable!()
}

fn part_2(mut cave: HashMap<(u32, u32), Material>, max_y: u32) -> u32 {
    for i in 0.. {
        let mut pos = (500, 0);
        loop {
            if cave.get(&(500, 0)).is_some() {
                return i;
            }
            match cave.get(&(pos.0, pos.1 + 1)).unwrap_or(&Material::Air) {
                Material::Air => {
                    if pos.1 + 1 > max_y + 1 {
                        break;
                    } else {
                        pos.1 += 1;
                    }
                }
                Material::Rock | Material::Sand => {
                    if cave.get(&(pos.0 - 1, pos.1 + 1)).is_none() {
                        pos.0 -= 1;
                        pos.1 += 1;
                    } else if cave.get(&(pos.0 + 1, pos.1 + 1)).is_none() {
                        pos.0 += 1;
                        pos.1 += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        cave.insert(pos, Material::Sand);
    }
    unreachable!()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let mut cave = HashMap::new();
    input.lines().for_each(|l| {
        let mut previous_point = None;
        let points = l.split(" -> ");
        points.for_each(|s| {
            let mut coords = s.split(',');
            let (x, y) = (
                coords.next().unwrap().parse::<u32>().unwrap(),
                coords.next().unwrap().parse::<u32>().unwrap(),
            );

            if let Some((prev_x, prev_y)) = previous_point {
                if x == prev_x {
                    for y in min(y, prev_y)..=max(y, prev_y) {
                        cave.insert((x, y), Material::Rock);
                    }
                } else {
                    for x in min(x, prev_x)..=max(x, prev_x) {
                        cave.insert((x, y), Material::Rock);
                    }
                }
            }

            previous_point = Some((x, y));
        });
    });

    let max_y = cave.keys().max_by(|lhs, rhs| lhs.1.cmp(&rhs.1)).unwrap().1;

    assert_eq!(part_1(cave.clone(), max_y), 592);
    assert_eq!(part_2(cave, max_y), 30367);
}
