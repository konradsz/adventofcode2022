use std::fs::read_to_string;

fn part_1(input: &str) -> usize {
    input
        .lines()
        .map(line_to_ranges)
        .filter(fully_overlapping)
        .count()
}

fn part_2(input: &str) -> usize {
    input
        .lines()
        .map(line_to_ranges)
        .filter(overlapping_at_all)
        .count()
}

struct Range {
    start: u64,
    end: u64,
}

fn fully_overlapping((lhs, rhs): &(Range, Range)) -> bool {
    (lhs.start <= rhs.start && lhs.end >= rhs.end) || (rhs.start <= lhs.start && rhs.end >= lhs.end)
}

fn overlapping_at_all((lhs, rhs): &(Range, Range)) -> bool {
    lhs.end >= rhs.start && lhs.start <= rhs.end || rhs.end >= lhs.start && rhs.start <= lhs.end
}

fn line_to_ranges(line: &str) -> (Range, Range) {
    let mut el = line.split(&[',', '-']).map(|el| el.parse::<u64>().unwrap());
    (
        Range {
            start: el.next().unwrap(),
            end: el.next().unwrap(),
        },
        Range {
            start: el.next().unwrap(),
            end: el.next().unwrap(),
        },
    )
}

fn main() {
    let input = read_to_string("input").unwrap();
    assert_eq!(part_1(&input), 507);
    assert_eq!(part_2(&input), 897);
}
