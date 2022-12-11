use std::collections::VecDeque;

fn part_1(program: VecDeque<Instruction>) -> i32 {
    let mut cpu = Cpu::new(program);
    (20..=220)
        .step_by(40)
        .map(|c| cpu.execute_next_cycles(c) * c as i32)
        .sum()
}

fn part_2(program: VecDeque<Instruction>) {
    let mut cpu = Cpu::new(program);

    let mut screen = vec![vec!['.'; 40]; 6];
    for row in 0..6 {
        for pixel in 0..40 {
            let sprite_position = cpu.execute_next_cycles((pixel + 1) + 40 * row);

            if pixel as i32 >= sprite_position - 1 && pixel as i32 <= sprite_position + 1 {
                screen[row as usize][(pixel) as usize] = '#';
            }
        }
    }

    for row in screen.iter() {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Add(u32, i32),
    Noop,
}

#[derive(Debug)]
struct Cpu {
    program: VecDeque<Instruction>,
    current_cycle: u32,
    register: i32,
}

impl Cpu {
    fn new(program: VecDeque<Instruction>) -> Self {
        Self {
            program,
            current_cycle: 0,
            register: 1,
        }
    }

    fn execute_next_cycles(&mut self, count: u32) -> i32 {
        let mut register = self.register;
        for _ in self.current_cycle..count {
            register = self.register;
            let instruction = self.program.pop_front().unwrap_or(Instruction::Noop);
            if let Instruction::Add(cycles, value) = instruction {
                if cycles == 1 {
                    self.register += value;
                } else {
                    self.program.push_front(Instruction::Add(1, value));
                }
            }

            self.current_cycle += 1;
        }
        register
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();

    let program = input
        .lines()
        .map(|line| {
            let mut parts = line.split_ascii_whitespace();
            let instruction = parts.next().unwrap();
            match instruction {
                "addx" => Instruction::Add(2, parts.next().unwrap().parse::<i32>().unwrap()),
                "noop" => Instruction::Noop,
                _ => panic!("unknown instruction"),
            }
        })
        .collect::<VecDeque<_>>();

    assert_eq!(part_1(program.clone()), 12880);
    part_2(program);
}
