use std::{
    cmp::{max, min},
    collections::HashMap,
};

#[derive(Debug)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

fn produce_sand(cave: &mut HashMap<(u32, u32), Material>, max_y: u32) -> bool {
    let mut current_position = (500, 0);

    // if let Some(m) = cave.get(&current_position) {
    //     if Material !=
    // }
    loop {
        if current_position.1 == max_y {
            return true;
        }
        match cave
            .get(&(current_position.0, current_position.1 + 1))
            .unwrap_or(&Material::Air)
        {
            Material::Air => current_position.1 += 1,
            Material::Rock | Material::Sand => {
                if cave
                    .get(&(current_position.0 - 1, current_position.1 + 1))
                    .is_none()
                {
                    current_position.0 -= 1;
                    current_position.1 += 1;
                } else if cave
                    .get(&(current_position.0 + 1, current_position.1 + 1))
                    .is_none()
                {
                    current_position.0 += 1;
                    current_position.1 += 1;
                } else {
                    break;
                }
            }
        }
    }
    cave.insert(current_position, Material::Sand);
    false
}

fn draw_cave(cave: &HashMap<(u32, u32), Material>, min_x: u32, max_x: u32, max_y: u32) {
    println!();
    for y in 0..=max_y {
        for x in min_x..=max_x {
            match cave.get(&(x, y)) {
                Some(m) => match m {
                    Material::Air => print!("."),
                    Material::Rock => print!("#"),
                    Material::Sand => print!("o"),
                },
                None => print!("."),
            }
        }
        println!();
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let structures = input
        .lines()
        .map(|l| {
            let points = l.split(" -> ");
            points
                .map(|s| {
                    let mut coords = s.split(',');
                    Point {
                        x: coords.next().unwrap().parse::<u32>().unwrap(),
                        y: coords.next().unwrap().parse::<u32>().unwrap(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let min_x = structures
        .iter()
        .map(|s| s.iter().min_by(|&p1, &p2| p1.x.cmp(&p2.x)).unwrap().x)
        .min()
        .unwrap();

    let max_x = structures
        .iter()
        .map(|s| s.iter().max_by(|&p1, &p2| p1.x.cmp(&p2.x)).unwrap().x)
        .max()
        .unwrap();

    let max_y = structures
        .iter()
        .map(|s| s.iter().max_by(|&p1, &p2| p1.y.cmp(&p2.y)).unwrap().y)
        .max()
        .unwrap();

    // println!("{min_x}, {max_x}, {max_y}");

    let mut cave = HashMap::new();
    for structure in structures {
        for (p1, p2) in structure.iter().zip(structure.iter().skip(1)) {
            // println!("{p1:?} -> {p2:?}");
            if p1.x == p2.x {
                for y in min(p1.y, p2.y)..=max(p1.y, p2.y) {
                    cave.insert((p1.x, y), Material::Rock);
                }
            } else {
                for x in min(p1.x, p2.x)..=max(p1.x, p2.x) {
                    cave.insert((x, p1.y), Material::Rock);
                }
            }
        }
    }

    // for _ in 0..25 {
    //     dbg!(produce_sand(&mut cave, max_y));
    //     draw_cave(&cave, min_x, max_x, max_y);
    // }
    for i in 0.. {
        if produce_sand(&mut cave, max_y) {
            println!("{i}");
            break;
        }
    }
}
