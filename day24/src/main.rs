use std::collections::{HashMap, HashSet, VecDeque};

const WIDTH: usize = 120;
const HEIGHT: usize = 25;

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

    fn move_once(&mut self) {
        match self.direction {
            Direction::Up => {
                if let Some(y) = self.coordinates.y.checked_sub(1) {
                    self.coordinates.y = y;
                } else {
                    self.coordinates.y = HEIGHT - 1;
                }
            }
            Direction::Down => {
                self.coordinates.y += 1;
                if self.coordinates.y == HEIGHT {
                    self.coordinates.y = 0;
                }
            }
            Direction::Left => {
                if let Some(x) = self.coordinates.x.checked_sub(1) {
                    self.coordinates.x = x;
                } else {
                    self.coordinates.x = WIDTH - 1;
                }
            }
            Direction::Right => {
                self.coordinates.x += 1;
                if self.coordinates.x == WIDTH {
                    self.coordinates.x = 0;
                }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    minute: usize,
    expedition: Option<Coordinates>,
}

impl State {
    fn new() -> Self {
        Self {
            minute: 0,
            expedition: None,
        }
    }

    fn from_state(other: Self) -> Self {
        Self {
            expedition: None,
            minute: other.minute,
            ..other
        }
    }

    fn get_expedition_possible_positions(
        &self,
        valley_entrance: Coordinates,
        blizzards: &Vec<Blizzard>,
    ) -> Vec<Option<Coordinates>> {
        if let Some(exp) = self.expedition {
            let mut positions = vec![];
            if self.can_move(exp, blizzards) {
                positions.push(Some(exp)); // don't move
            }
            if exp.y != 0 && self.can_move(Coordinates::new(exp.x, exp.y - 1), blizzards) {
                positions.push(Some(Coordinates::new(exp.x, exp.y - 1))); // go up
            }
            if exp.y != HEIGHT - 1 && self.can_move(Coordinates::new(exp.x, exp.y + 1), blizzards) {
                positions.push(Some(Coordinates::new(exp.x, exp.y + 1))); // go down
            }
            if exp.x != 0 && self.can_move(Coordinates::new(exp.x - 1, exp.y), blizzards) {
                positions.push(Some(Coordinates::new(exp.x - 1, exp.y))); // go left
            }
            if exp.x != WIDTH - 1 && self.can_move(Coordinates::new(exp.x + 1, exp.y), blizzards) {
                positions.push(Some(Coordinates::new(exp.x + 1, exp.y))); // go right
            }
            positions
        } else if self.can_move(valley_entrance, blizzards) {
            vec![None, Some(valley_entrance)] // wait before entering valley, enter valley
        } else {
            vec![None] // wait before entering valley
        }
    }

    fn can_move(&self, coordinates: Coordinates, blizzards: &Vec<Blizzard>) -> bool {
        blizzards.iter().all(|b| b.coordinates != coordinates)
    }
}

fn measure_quickest_path(
    mut initial_state: State,
    valley_entrance: Coordinates,
    valley_exit: Coordinates,
    blizzard_history: &mut HashMap<usize, Vec<Blizzard>>,
) -> State {
    initial_state.minute += 1;
    let mut states = VecDeque::new();
    states.push_back(initial_state);
    let mut seen_states: HashSet<State> = HashSet::new();
    while let Some(mut state) = states.pop_front() {
        if seen_states.get(&state).is_some() {
            continue;
        }
        seen_states.insert(state.clone());
        if let Some(exp) = state.expedition {
            if exp == valley_exit {
                state.expedition = None;
                return state;
            }
        }

        let blizzards = if let Some(blizzards) = blizzard_history.get(&state.minute) {
            blizzards
        } else {
            let mut blizzard = blizzard_history.get(&(state.minute - 1)).unwrap().clone();
            blizzard.iter_mut().for_each(|b| b.move_once());
            blizzard_history.insert(state.minute, blizzard);
            blizzard_history.get(&state.minute).unwrap()
        };
        let positions = state.get_expedition_possible_positions(valley_entrance, blizzards);
        for pos in positions {
            let mut s = state.clone();
            s.expedition = pos;
            s.minute += 1;

            states.push_back(s);
        }
    }
    panic!("cannot traverse")
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let input = input.lines().collect::<Vec<_>>();

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

    let mut blizzard_history: HashMap<usize, Vec<Blizzard>> = HashMap::new();
    blizzard_history.insert(0, blizzards);
    let state = measure_quickest_path(
        State::new(),
        Coordinates::new(0, 0),
        Coordinates::new(WIDTH - 1, HEIGHT - 1),
        &mut blizzard_history,
    );

    let part_1 = state.minute;
    assert_eq!(part_1, 274);

    let state = measure_quickest_path(
        State::from_state(state),
        Coordinates::new(WIDTH - 1, HEIGHT - 1),
        Coordinates::new(0, 0),
        &mut blizzard_history,
    );

    let state = measure_quickest_path(
        State::from(state),
        Coordinates::new(0, 0),
        Coordinates::new(WIDTH - 1, HEIGHT - 1),
        &mut blizzard_history,
    );

    let part_2 = state.minute;
    assert_eq!(part_2, 839);
}
