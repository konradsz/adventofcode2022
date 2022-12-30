use std::collections::VecDeque;

#[derive(Clone)]
struct Monkey {
    items: VecDeque<u64>,
    operation: fn(u64) -> u64,
    test: u64,
    when_true: usize,
    when_false: usize,
    inspected_items: u64,
}

impl Monkey {
    fn new(
        items: VecDeque<u64>,
        operation: fn(u64) -> u64,
        test: u64,
        when_true: usize,
        when_false: usize,
    ) -> Self {
        Self {
            items,
            operation,
            test,
            when_true,
            when_false,
            inspected_items: 0,
        }
    }
}

#[derive(Clone)]
struct Monkeys(Vec<Monkey>);

impl Monkeys {
    fn play_round(&mut self, after_inspect: fn(u64) -> u64) {
        let mut current_monkey = 0;
        loop {
            if let Some(mut item) = self.0[current_monkey].items.pop_front() {
                self.0[current_monkey].inspected_items += 1;
                item = (self.0[current_monkey].operation)(item);
                item = after_inspect(item);
                if item % self.0[current_monkey].test == 0 {
                    let new_owner = self.0[current_monkey].when_true;
                    self.0[new_owner].items.push_back(item);
                } else {
                    let new_owner = self.0[current_monkey].when_false;
                    self.0[new_owner].items.push_back(item);
                }
            } else {
                current_monkey += 1;
                if current_monkey == self.0.len() {
                    break;
                }
            }
        }
    }
}

fn part_1(mut monkeys: Monkeys) -> u64 {
    for _ in 0..20 {
        monkeys.play_round(|worry_level| worry_level / 3);
    }

    monkeys
        .0
        .sort_by(|m1, m2| m2.inspected_items.cmp(&m1.inspected_items));
    monkeys.0[0].inspected_items * monkeys.0[1].inspected_items
}

fn part_2(mut monkeys: Monkeys) -> u64 {
    for _ in 0..10000 {
        monkeys.play_round(|worry_level| worry_level % (2 * 17 * 19 * 3 * 5 * 13 * 7 * 11))
    }

    monkeys
        .0
        .sort_by(|m1, m2| m2.inspected_items.cmp(&m1.inspected_items));
    monkeys.0[0].inspected_items * monkeys.0[1].inspected_items
}

fn main() {
    let monkeys = vec![
        Monkey::new(VecDeque::from([83, 62, 93]), |old| old * 17, 2, 1, 6),
        Monkey::new(VecDeque::from([90, 55]), |old| old + 1, 17, 6, 3),
        Monkey::new(
            VecDeque::from([91, 78, 80, 97, 79, 88]),
            |old| old + 3,
            19,
            7,
            5,
        ),
        Monkey::new(VecDeque::from([64, 80, 83, 89, 59]), |old| old + 5, 3, 7, 2),
        Monkey::new(VecDeque::from([98, 92, 99, 51]), |old| old * old, 5, 0, 1),
        Monkey::new(
            VecDeque::from([68, 57, 95, 85, 98, 75, 98, 75]),
            |old| old + 2,
            13,
            4,
            0,
        ),
        Monkey::new(VecDeque::from([74]), |old| old + 4, 7, 3, 2),
        Monkey::new(
            VecDeque::from([68, 64, 60, 68, 87, 80, 82]),
            |old| old * 19,
            11,
            4,
            5,
        ),
    ];

    let monkeys = Monkeys(monkeys);

    assert_eq!(part_1(monkeys.clone()), 112815);
    assert_eq!(part_2(monkeys), 25738411485);
}
