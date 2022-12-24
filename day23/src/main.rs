use std::{
    cmp::{max, min},
    collections::HashSet,
};

const DIRECTIONS: [[(i64, i64); 3]; 4] = [
    [(-1, -1), (0, -1), (1, -1)],
    [(-1, 1), (0, 1), (1, 1)],
    [(-1, -1), (-1, 0), (-1, 1)],
    [(1, -1), (1, 0), (1, 1)],
];

const NEIGHBOURS: [(i64, i64); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    fn move_by(self, offset: (i64, i64)) -> Self {
        Coordinates {
            x: self.x + offset.0,
            y: self.y + offset.1,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Elf {
    position: Coordinates,
    proposed_position: Option<Coordinates>,
}

impl Elf {
    fn new(x: i64, y: i64) -> Self {
        Self {
            position: Coordinates { x, y },
            proposed_position: None,
        }
    }

    fn propose_new_position(&mut self, occupied_positions: &HashSet<Coordinates>, turn: usize) {
        if NEIGHBOURS.iter().all(|n| {
            occupied_positions
                .get(&Coordinates {
                    x: self.position.x + n.0,
                    y: self.position.y + n.1,
                })
                .is_none()
        }) {
            return;
        }
        for i in 0..4 {
            let dir = DIRECTIONS[(turn + i) % DIRECTIONS.len()];
            if occupied_positions
                .get(&self.position.move_by(dir[0]))
                .is_none()
                && occupied_positions
                    .get(&self.position.move_by(dir[1]))
                    .is_none()
                && occupied_positions
                    .get(&self.position.move_by(dir[2]))
                    .is_none()
            {
                self.proposed_position = Some(self.position.move_by(dir[1]));
                break;
            }
        }
    }

    fn move_if_allowed(&mut self, conflicted_positions: &[Coordinates]) {
        let position_candidate = self.proposed_position.take();
        if let Some(position_candidate) = position_candidate {
            if !conflicted_positions.contains(&position_candidate) {
                self.position = position_candidate;
            }
        }
    }
}

fn count_empty(positions: &HashSet<Coordinates>) -> usize {
    let ((min_x, max_x), (min_y, max_y)) = positions.iter().fold(
        ((i64::MAX, i64::MIN), (i64::MAX, i64::MIN)),
        |((min_x, max_x), (min_y, max_y)), coord| {
            (
                (min(min_x, coord.x), max(max_x, coord.x)),
                (min(min_y, coord.y), max(max_y, coord.y)),
            )
        },
    );

    (min_y..=max_y)
        .map(|y| {
            (min_x..=max_x)
                .filter(|&x| positions.get(&Coordinates { x, y }).is_none())
                .count()
        })
        .sum()
}

fn simulate_round(elves: &mut [Elf], turn: usize) -> bool {
    let occupied_positions = elves.iter().map(|elf| elf.position).collect::<HashSet<_>>();
    elves
        .iter_mut()
        .for_each(|el| el.propose_new_position(&occupied_positions, turn));
    let mut proposed_positions = HashSet::new();
    let conflicted_positions = elves
        .iter()
        .filter_map(|elf| elf.proposed_position)
        .filter(|pp| !proposed_positions.insert(*pp))
        .collect::<Vec<_>>();

    for elf in elves.iter_mut() {
        elf.move_if_allowed(&conflicted_positions);
    }

    let positions = elves.iter().map(|elf| elf.position).collect::<HashSet<_>>();
    occupied_positions == positions
}

fn part_1(mut elves: Vec<Elf>) -> usize {
    for turn in 0..10 {
        simulate_round(&mut elves, turn);
    }
    count_empty(&elves.into_iter().map(|elf| elf.position).collect())
}

fn part_2(mut elves: Vec<Elf>) -> usize {
    (0..)
        .take_while(|turn| !simulate_round(&mut elves, *turn))
        .count()
        + 1
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let elves = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(|(x, _)| Elf::new(x as i64, y as i64))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    assert_eq!(part_1(elves.clone()), 3762);
    assert_eq!(part_2(elves), 997);
}
