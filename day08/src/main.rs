use std::{collections::HashSet, fs::read_to_string};

fn part_1(trees: &[Vec<u32>]) -> usize {
    let mut visible = HashSet::new();
    let grid_size = trees.len();

    look_from_left(trees, &mut visible);
    look_from_right(trees, &mut visible);
    look_from_top(trees, &mut visible);
    look_from_bottom(trees, &mut visible);

    visible.len() + 4 * grid_size - 4
}

fn part_2(trees: &[Vec<u32>]) -> usize {
    let grid_size = trees.len();
    let mut max = 0;
    for y in 1..grid_size - 1 {
        for x in 1..grid_size - 1 {
            let right = look_right(trees, x, y);
            let left = look_left(trees, x, y);
            let top = look_up(trees, y, x);
            let bottom = look_down(trees, y, x);
            let score = right * left * top * bottom;
            if score > max {
                max = score;
            }
        }
    }

    max
}

fn look_from_left(trees: &[Vec<u32>], visible: &mut HashSet<(usize, usize)>) {
    let grid_size = trees.len();
    for y in 1..grid_size - 1 {
        let mut current_heighest = trees[y][0];
        for x in 1..grid_size - 1 {
            if trees[y][x] > current_heighest {
                visible.insert((x, y));
                current_heighest = trees[y][x]
            }
        }
    }
}

fn look_from_right(trees: &[Vec<u32>], visible: &mut HashSet<(usize, usize)>) {
    let grid_size = trees.len();
    for y in 1..grid_size - 1 {
        let mut current_heighest = trees[y][grid_size - 1];
        for x in (1..grid_size - 1).rev() {
            if trees[y][x] > current_heighest {
                visible.insert((x, y));
                current_heighest = trees[y][x]
            }
        }
    }
}

fn look_from_top(trees: &[Vec<u32>], visible: &mut HashSet<(usize, usize)>) {
    let grid_size = trees.len();
    for x in 1..grid_size - 1 {
        let mut current_heighest = trees[0][x];
        for y in 1..grid_size - 1 {
            if trees[y][x] > current_heighest {
                visible.insert((x, y));
                current_heighest = trees[y][x];
            }
        }
    }
}

fn look_from_bottom(trees: &[Vec<u32>], visible: &mut HashSet<(usize, usize)>) {
    let grid_size = trees.len();
    for x in 1..grid_size - 1 {
        let mut current_heighest = trees[grid_size - 1][x];
        for y in (1..grid_size - 1).rev() {
            if trees[y][x] > current_heighest {
                visible.insert((x, y));
                current_heighest = trees[y][x];
            }
        }
    }
}

fn look_right(trees: &[Vec<u32>], init_x: usize, y: usize) -> usize {
    let grid_size = trees.len();
    (init_x + 1..grid_size)
        .enumerate()
        .find(|(_, x)| trees[y][*x] >= trees[y][init_x])
        .map(|(count, _)| count + 1)
        .unwrap_or(grid_size - init_x - 1)
}

fn look_left(trees: &[Vec<u32>], init_x: usize, y: usize) -> usize {
    (0..=init_x - 1)
        .rev()
        .enumerate()
        .find(|(_, x)| trees[y][*x] >= trees[y][init_x])
        .map(|(count, _)| count + 1)
        .unwrap_or(init_x)
}

fn look_up(trees: &[Vec<u32>], init_y: usize, x: usize) -> usize {
    (0..=init_y - 1)
        .rev()
        .enumerate()
        .find(|(_, y)| trees[*y][x] >= trees[init_y][x])
        .map(|(count, _)| count + 1)
        .unwrap_or(init_y)
}

fn look_down(trees: &[Vec<u32>], init_y: usize, x: usize) -> usize {
    let grid_size = trees.len();
    (init_y + 1..grid_size)
        .enumerate()
        .find(|(_, y)| trees[*y][x] >= trees[init_y][x])
        .map(|(count, _)| count + 1)
        .unwrap_or(grid_size - init_y - 1)
}

fn main() {
    let input = read_to_string("input").unwrap();
    let trees = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    assert_eq!(part_1(&trees), 1703);
    assert_eq!(part_2(&trees), 496650);
}
