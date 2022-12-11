use std::collections::VecDeque;

#[derive(Debug)]
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
        .collect();

    let mut cpu = Cpu {
        program,
        current_cycle: 0,
        register: 1,
    };

    let mut sum = 0;
    for count in (20..=220).step_by(40) {
        let register = cpu.execute_next_cycles(count);
        println!("{} * {}", count, register);
        sum += count as i32 * register;
    }
    println!("{sum}");
}
