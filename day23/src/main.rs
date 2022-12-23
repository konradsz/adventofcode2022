use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i64,
    y: i64,
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
        let neighbours = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        if neighbours.iter().all(|n| {
            occupied_positions
                .get(&Coordinates {
                    x: self.position.x + n.0,
                    y: self.position.y + n.1,
                })
                .is_none()
        }) {
            return;
        }
        let directions = [
            [(-1, -1), (0, -1), (1, -1)],
            [(-1, 1), (0, 1), (1, 1)],
            [(-1, -1), (-1, 0), (-1, 1)],
            [(1, -1), (1, 0), (1, 1)],
        ];
        for i in 0..4 {
            let dir = directions[(turn + i) % directions.len()];
            let p1 = Coordinates {
                x: self.position.x + dir[0].0,
                y: self.position.y + dir[0].1,
            };
            let p2 = Coordinates {
                x: self.position.x + dir[1].0,
                y: self.position.y + dir[1].1,
            };
            let p3 = Coordinates {
                x: self.position.x + dir[2].0,
                y: self.position.y + dir[2].1,
            };
            if occupied_positions.get(&p1).is_none()
                && occupied_positions.get(&p2).is_none()
                && occupied_positions.get(&p3).is_none()
            {
                self.proposed_position = Some(p2);
                break;
            }
        }
    }

    fn move_if_allowed(&mut self, conflicted_positions: &Vec<Coordinates>) {
        let position_candidate = self.proposed_position.take();
        if let Some(position_candidate) = position_candidate {
            if !conflicted_positions.contains(&position_candidate) {
                self.position = position_candidate;
            }
        }
    }
}

fn count_empty(positions: &HashSet<Coordinates>) -> usize {
    let min_x = positions
        .iter()
        .min_by(|lhs, rhs| lhs.x.cmp(&rhs.x))
        .unwrap()
        .x;
    let max_x = positions
        .iter()
        .max_by(|lhs, rhs| lhs.x.cmp(&rhs.x))
        .unwrap()
        .x;
    let min_y = positions
        .iter()
        .min_by(|lhs, rhs| lhs.y.cmp(&rhs.y))
        .unwrap()
        .y;
    let max_y = positions
        .iter()
        .max_by(|lhs, rhs| lhs.y.cmp(&rhs.y))
        .unwrap()
        .y;

    let mut empty = 0;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if positions.get(&Coordinates { x, y }).is_none() {
                empty += 1;
            }
        }
    }
    empty
}

fn simulate_round(elves: &mut Vec<Elf>, turn: usize) -> bool {
    let occupied_positions = elves.iter().map(|elf| elf.position).collect::<HashSet<_>>();
    elves
        .iter_mut()
        .for_each(|el| el.propose_new_position(&occupied_positions, turn));
    let mut proposed_positions = HashSet::new();
    let mut conflicted_positions = Vec::new();
    for elf in elves.iter() {
        if let Some(pp) = elf.proposed_position {
            if !proposed_positions.insert(pp) {
                conflicted_positions.push(pp);
            }
        }
    }

    for elf in elves.iter_mut() {
        elf.move_if_allowed(&conflicted_positions);
    }

    let positions = elves.iter().map(|elf| elf.position).collect::<HashSet<_>>();
    count_empty(&positions);

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

    let mut elves = Vec::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.push(Elf::new(x as i64, y as i64));
            }
        }
    }

    assert_eq!(part_1(elves.clone()), 3762);
    assert_eq!(part_2(elves.clone()), 997);
}
