use std::collections::HashSet;

fn part_1(input: &str) -> usize {
    let mut rope = Rope::with_knots(2);
    process_motions(input, &mut rope)
}

fn part_2(input: &str) -> usize {
    let mut rope = Rope::with_knots(10);
    process_motions(input, &mut rope)
}

fn process_motions(input: &str, rope: &mut Rope) -> usize {
    let mut visited = HashSet::new();
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let direction = parts.next().unwrap();
        let count = parts.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..count {
            match direction {
                "R" => rope.move_right(),
                "L" => rope.move_left(),
                "U" => rope.move_up(),
                "D" => rope.move_down(),
                _ => panic!(),
            }
            visited.insert(*rope.knots.last().unwrap());
        }
    }

    visited.len()
}

#[derive(Default, Debug)]
struct Rope {
    knots: Vec<(i32, i32)>,
}

impl Rope {
    fn with_knots(size: usize) -> Self {
        Self {
            knots: vec![(0, 0); size],
        }
    }
    fn move_right(&mut self) {
        self.knots[0].0 += 1;
        for i in 1..self.knots.len() {
            self.move_knot_if_needed(i);
        }
    }

    fn move_left(&mut self) {
        self.knots[0].0 -= 1;
        for i in 1..self.knots.len() {
            self.move_knot_if_needed(i);
        }
    }

    fn move_up(&mut self) {
        self.knots[0].1 += 1;
        for i in 1..self.knots.len() {
            self.move_knot_if_needed(i);
        }
    }

    fn move_down(&mut self) {
        self.knots[0].1 -= 1;
        for i in 1..self.knots.len() {
            self.move_knot_if_needed(i);
        }
    }

    fn move_knot_if_needed(&mut self, idx: usize) {
        let v = (
            self.knots[idx - 1].0 - self.knots[idx].0,
            self.knots[idx - 1].1 - self.knots[idx].1,
        );
        let length = f64::from(v.0.pow(2) + v.1.pow(2)).sqrt().round() as i32;
        if length > 1 {
            let off = (
                std::cmp::min(1, v.0.abs()) * v.0.signum(),
                std::cmp::min(1, v.1.abs()) * v.1.signum(),
            );

            self.knots[idx].0 += off.0;
            self.knots[idx].1 += off.1;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    assert_eq!(part_1(&input), 6391);
    assert_eq!(part_2(&input), 2593);
}
