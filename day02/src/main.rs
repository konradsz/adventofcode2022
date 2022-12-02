use std::{collections::HashMap, fs::read_to_string};

fn part_1(input: &str) -> i64 {
    let scores = HashMap::from([
        ("A X", 4),
        ("A Y", 8),
        ("A Z", 3),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 7),
        ("C Y", 2),
        ("C Z", 6),
    ]);
    input.lines().map(|l| scores.get(l).unwrap()).sum()
}

fn part_2(input: &str) -> i64 {
    let scores = HashMap::from([
        ("A X", 3),
        ("A Y", 4),
        ("A Z", 8),
        ("B X", 1),
        ("B Y", 5),
        ("B Z", 9),
        ("C X", 2),
        ("C Y", 6),
        ("C Z", 7),
    ]);
    input.lines().map(|l| scores.get(l).unwrap()).sum()
}

fn main() {
    let input = read_to_string("input").unwrap();

    assert_eq!(part_1(&input), 13009);
    assert_eq!(part_2(&input), 10398);
}
