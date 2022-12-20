use std::collections::VecDeque;

fn part_1(numbers: &[i64]) -> i64 {
    let positions = (0..numbers.len()).collect::<VecDeque<_>>();
    let positions = mix(numbers, positions);

    find_coordinates(numbers, &positions)
}

fn part_2(numbers: &[i64]) -> i64 {
    let numbers = numbers.iter().map(|n| n * 811589153).collect::<Vec<i64>>();
    let mut positions = (0..numbers.len()).collect::<VecDeque<_>>();

    for _ in 0..10 {
        positions = mix(&numbers, positions);
    }

    find_coordinates(&numbers, &positions)
}

fn mix(numbers: &[i64], mut positions: VecDeque<usize>) -> VecDeque<usize> {
    for (idx, &number) in numbers.iter().enumerate() {
        if number == 0 {
            continue;
        }

        let index = positions.iter().position(|&pos| pos == idx).unwrap();
        let position = positions.remove(index).unwrap();

        if number > 0 {
            let value = number as usize % (numbers.len() - 1);
            positions.rotate_left(value);
            positions.insert(index, position);
        }
        if number < 0 {
            let value = number.unsigned_abs() as usize % (numbers.len() - 1);
            positions.rotate_right(value);
            positions.insert(index, position);
        }
    }

    positions
}

fn find_coordinates(numbers: &[i64], positions: &VecDeque<usize>) -> i64 {
    let position_of_zero = numbers.iter().position(|n| n == &0).unwrap();
    let idx_of_zero = positions
        .iter()
        .position(|&p| p == position_of_zero)
        .unwrap();
    (1..=3)
        .map(|f| numbers[positions[(idx_of_zero + f * 1000) % numbers.len()]])
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let numbers = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(part_1(&numbers), 8721);
    assert_eq!(part_2(&numbers), 831878881825);
}
