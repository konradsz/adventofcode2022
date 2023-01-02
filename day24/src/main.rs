use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Blizzard {
    coordinates: Coordinates,
    direction: Direction,
}

impl Blizzard {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self {
            coordinates: Coordinates { x, y },
            direction,
        }
    }

    fn move_once(&mut self, width: usize, height: usize) {
        match self.direction {
            Direction::Up => {
                if let Some(y) = self.coordinates.y.checked_sub(1) {
                    self.coordinates.y = y;
                } else {
                    self.coordinates.y = height - 1;
                }
            }
            Direction::Down => {
                self.coordinates.y += 1;
                if self.coordinates.y == height {
                    self.coordinates.y = 0;
                }
            }
            Direction::Left => {
                if let Some(x) = self.coordinates.x.checked_sub(1) {
                    self.coordinates.x = x;
                } else {
                    self.coordinates.x = width - 1;
                }
            }
            Direction::Right => {
                self.coordinates.x += 1;
                if self.coordinates.x == width {
                    self.coordinates.x = 0;
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Map {
    width: usize,
    height: usize,
    expedition: Option<Coordinates>,
    blizzards: Vec<Blizzard>,
}

impl Map {
    fn new(width: usize, height: usize, blizzards: Vec<Blizzard>) -> Self {
        Self {
            width,
            height,
            expedition: None,
            blizzards,
        }
    }

    fn move_blizzards(&mut self) {
        self.blizzards
            .iter_mut()
            .for_each(|b| b.move_once(self.width, self.height));
    }

    fn get_expedition_possible_positions(&self) -> Vec<Option<Coordinates>> {
        if let Some(exp) = self.expedition {
            let mut positions = vec![];
            if self.get_blizzard_count(exp) == 0 {
                positions.push(Some(exp)); // don't move
            }
            if exp.y != 0 && self.get_blizzard_count(Coordinates::new(exp.x, exp.y - 1)) == 0 {
                positions.push(Some(Coordinates::new(exp.x, exp.y - 1))); // go up
            }
            if exp.y != self.height - 1
                && self.get_blizzard_count(Coordinates::new(exp.x, exp.y + 1)) == 0
            {
                positions.push(Some(Coordinates::new(exp.x, exp.y + 1))); // go down
            }
            if exp.x != 0 && self.get_blizzard_count(Coordinates::new(exp.x - 1, exp.y)) == 0 {
                positions.push(Some(Coordinates::new(exp.x - 1, exp.y))); // go left
            }
            if exp.x != self.width - 1
                && self.get_blizzard_count(Coordinates::new(exp.x + 1, exp.y)) == 0
            {
                positions.push(Some(Coordinates::new(exp.x + 1, exp.y))); // go right
            }
            positions
        } else if self.get_blizzard_count(Coordinates::new(0, 0)) == 0 {
            vec![None, Some(Coordinates::new(0, 0))]
        } else {
            vec![None]
        }
    }

    fn get_expedition_possible_positions2(&self) -> Vec<Option<Coordinates>> {
        if let Some(exp) = self.expedition {
            let mut positions = vec![];
            if self.get_blizzard_count(exp) == 0 {
                positions.push(Some(exp)); // don't move
            }
            if exp.y != 0 && self.get_blizzard_count(Coordinates::new(exp.x, exp.y - 1)) == 0 {
                positions.push(Some(Coordinates::new(exp.x, exp.y - 1))); // go up
            }
            if exp.y != self.height - 1
                && self.get_blizzard_count(Coordinates::new(exp.x, exp.y + 1)) == 0
            {
                positions.push(Some(Coordinates::new(exp.x, exp.y + 1))); // go down
            }
            if exp.x != 0 && self.get_blizzard_count(Coordinates::new(exp.x - 1, exp.y)) == 0 {
                positions.push(Some(Coordinates::new(exp.x - 1, exp.y))); // go left
            }
            if exp.x != self.width - 1
                && self.get_blizzard_count(Coordinates::new(exp.x + 1, exp.y)) == 0
            {
                positions.push(Some(Coordinates::new(exp.x + 1, exp.y))); // go right
            }
            positions
        } else if self.get_blizzard_count(Coordinates::new(self.width - 1, self.height - 1)) == 0 {
            vec![
                None,
                Some(Coordinates::new(self.width - 1, self.height - 1)),
            ]
        } else {
            vec![None]
        }
    }

    fn get_blizzard_count(&self, coordinates: Coordinates) -> usize {
        self.blizzards
            .iter()
            .filter(|b| b.coordinates == coordinates)
            .count()
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let input = input.lines().collect::<Vec<_>>();
    let width = input[0].len() - 2;
    let height = input.len() - 2;

    let mut blizzards = vec![];
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let (x, y) = (x - 1, y - 1);
            match c {
                '^' => blizzards.push(Blizzard::new(x, y, Direction::Up)),
                'v' => blizzards.push(Blizzard::new(x, y, Direction::Down)),
                '<' => blizzards.push(Blizzard::new(x, y, Direction::Left)),
                '>' => blizzards.push(Blizzard::new(x, y, Direction::Right)),
                _ => (),
            }
        }
    }

    let mut map = Map::new(width, height, blizzards);

    let mut states = VecDeque::new();
    states.push_back((1, map.clone()));
    let mut seen_states: HashSet<Map> = HashSet::new();
    let mut f = map.clone();
    while let Some((minute, mut map)) = states.pop_front() {
        if let Some(ss) = seen_states.get(&map) {
            continue;
        }
        seen_states.insert(map.clone());
        if let Some(exp) = map.expedition {
            if exp == Coordinates::new(map.width - 1, map.height - 1) {
                println!("FOUND! {minute}");
                f = map;
                break;
            }
        }

        map.move_blizzards();
        let positions = map.get_expedition_possible_positions();
        for pos in positions {
            let mut m = map.clone();
            m.expedition = pos;

            states.push_back((minute + 1, m));
        }
    }

    states.clear();
    seen_states.clear();
    f.expedition = None;
    f.move_blizzards();
    states.push_back((1, f));

    let mut ff = map.clone();
    while let Some((minute, mut map)) = states.pop_front() {
        if let Some(ss) = seen_states.get(&map) {
            continue;
        }
        seen_states.insert(map.clone());
        if let Some(exp) = map.expedition {
            if exp == Coordinates::new(0, 0) {
                println!("FOUND! {minute}");
                ff = map;
                break;
            }
        }

        map.move_blizzards();
        let positions = map.get_expedition_possible_positions2();
        for pos in positions {
            let mut m = map.clone();
            m.expedition = pos;

            states.push_back((minute + 1, m));
        }
    }

    states.clear();
    seen_states.clear();
    ff.expedition = None;
    ff.move_blizzards();
    states.push_back((1, ff));
    while let Some((minute, mut map)) = states.pop_front() {
        if let Some(ss) = seen_states.get(&map) {
            continue;
        }
        seen_states.insert(map.clone());
        if let Some(exp) = map.expedition {
            if exp == Coordinates::new(map.width - 1, map.height - 1) {
                println!("FOUND! {minute}");
                break;
            }
        }

        map.move_blizzards();
        let positions = map.get_expedition_possible_positions();
        for pos in positions {
            let mut m = map.clone();
            m.expedition = pos;

            states.push_back((minute + 1, m));
        }
    }
}
