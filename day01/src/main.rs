use std::{collections::BTreeSet, fs::read_to_string};

fn part_1(input: &str) -> i64 {
    input
        .lines()
        .fold((0, 0), |(current, max), line| {
            if line.is_empty() {
                if current > max {
                    (0, current)
                } else {
                    (0, max)
                }
            } else {
                (current + line.parse::<i64>().unwrap(), max)
            }
        })
        .1
}

fn part_2(input: &str) -> i64 {
    input
        .lines()
        .fold((0, BTreeSet::new()), |(current, mut set), line| {
            if line.is_empty() {
                set.insert(current);
                (0, set)
            } else {
                (current + line.parse::<i64>().unwrap(), set)
            }
        })
        .1
        .iter()
        .rev()
        .take(3)
        .sum()
}

fn main() {
    let input = read_to_string("input").unwrap();

    assert_eq!(part_1(&input), 75501);
    assert_eq!(part_2(&input), 215594);
}
