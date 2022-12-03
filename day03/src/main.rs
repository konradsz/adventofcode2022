use std::{collections::HashSet, fs::read_to_string};

fn part_1(input: &[&[u8]]) -> u64 {
    input
        .iter()
        .map(|line| {
            let rucksack_size = line.len() / 2;
            let shared_item = find_shared_item(&[&line[0..rucksack_size], &line[rucksack_size..]]);
            get_priority(shared_item)
        })
        .sum()
}

fn part_2(input: &[&[u8]]) -> u64 {
    input
        .chunks(3)
        .map(|rs| {
            let shared_item = find_shared_item(&[rs[0], rs[1], rs[2]]);
            get_priority(shared_item)
        })
        .sum()
}

fn find_shared_item(rucksacks: &[&[u8]]) -> u8 {
    rucksacks
        .iter()
        .map(|r| HashSet::<u8>::from_iter(r.iter().copied()))
        .reduce(|acc, el| acc.intersection(&el).copied().collect())
        .unwrap()
        .into_iter()
        .next()
        .unwrap()
}

fn get_priority(item: u8) -> u64 {
    if item.is_ascii_lowercase() {
        (item - b'a' + 1) as u64
    } else {
        (item - b'A' + 27) as u64
    }
}

fn main() {
    let input = read_to_string("input").unwrap();
    let input = input.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();

    assert_eq!(part_1(&input), 7742);
    assert_eq!(part_2(&input), 2276);
}
