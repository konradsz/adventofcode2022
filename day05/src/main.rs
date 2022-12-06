use std::fs::read_to_string;

fn part_1(stacks: Vec<Vec<char>>, instructions: &[Instruction]) -> String {
    let mut stacks = instructions.iter().fold(stacks, |mut s, i| {
        for _ in 0..i.count {
            let c = s[i.from - 1].pop().unwrap();
            s[i.to - 1].push(c);
        }
        s
    });

    stacks.iter_mut().map(|s| s.pop().unwrap()).collect()
}

fn part_2(stacks: Vec<Vec<char>>, instructions: &[Instruction]) -> String {
    let mut stacks = instructions.iter().fold(stacks, |mut s, i| {
        let h = s[i.from - 1].len();
        let mut to_move = s[i.from - 1].split_off(h - i.count);
        s[i.to - 1].append(&mut to_move);
        s
    });

    stacks.iter_mut().map(|s| s.pop().unwrap()).collect()
}

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let input = read_to_string("input").unwrap();

    let mut instructions = vec![];
    for line in input.lines() {
        let mut parts = line.split_whitespace();
        parts.next();
        let count = parts.next().unwrap().parse().unwrap();
        parts.next();
        let from = parts.next().unwrap().parse().unwrap();
        parts.next();
        let to = parts.next().unwrap().parse().unwrap();

        instructions.push(Instruction { count, from, to });
    }

    let stacks = vec![
        vec!['Z', 'T', 'F', 'R', 'W', 'J', 'G'],
        vec!['G', 'W', 'M'],
        vec!['J', 'N', 'H', 'G'],
        vec!['J', 'R', 'C', 'N', 'W'],
        vec!['W', 'F', 'S', 'B', 'G', 'Q', 'V', 'M'],
        vec!['S', 'R', 'T', 'D', 'V', 'W', 'C'],
        vec!['H', 'B', 'N', 'C', 'D', 'Z', 'G', 'V'],
        vec!['S', 'J', 'N', 'M', 'G', 'C'],
        vec!['G', 'P', 'N', 'W', 'C', 'J', 'D', 'L'],
    ];

    assert_eq!(part_1(stacks.clone(), &instructions), "CWMTGHBDW");
    assert_eq!(part_2(stacks.clone(), &instructions), "SSCGWJCRB");
}
