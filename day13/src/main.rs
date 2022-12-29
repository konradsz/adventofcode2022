use std::{cmp::Ordering, collections::VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketElement {
    List(Vec<PacketElement>),
    Integer(u32),
}

impl PartialOrd for PacketElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PacketElement {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketElement::List(lhs), PacketElement::List(rhs)) => {
                for (l, r) in lhs.iter().zip(rhs.iter()) {
                    let res = l.cmp(r);
                    if res.is_ne() {
                        return res;
                    }
                }
                lhs.len().cmp(&rhs.len())
            }
            (PacketElement::List(_), PacketElement::Integer(_)) => {
                self.cmp(&PacketElement::List(vec![other.clone()]))
            }
            (PacketElement::Integer(_), PacketElement::List(_)) => {
                PacketElement::List(vec![self.clone()]).cmp(other)
            }
            (PacketElement::Integer(lhs), PacketElement::Integer(rhs)) => lhs.cmp(rhs),
        }
    }
}

fn create_list(input: &mut VecDeque<char>) -> Vec<PacketElement> {
    let mut result = vec![];
    loop {
        let front = input.pop_front().unwrap();
        match front {
            '[' => result.push(PacketElement::List(create_list(input))),
            ',' | ' ' => (),
            '1' => {
                let next = input.front().unwrap();
                if next == &'0' {
                    input.pop_front();
                    result.push(PacketElement::Integer(10));
                } else {
                    result.push(PacketElement::Integer(1));
                }
            }
            '0' | '2'..='9' => {
                result.push(PacketElement::Integer(front.to_digit(10).unwrap()));
            }
            ']' => {
                return result;
            }
            _ => panic!(),
        }
    }
}

fn build(input: &mut VecDeque<char>) -> PacketElement {
    input.pop_front();
    return PacketElement::List(create_list(input));
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let input = input.lines().collect::<Vec<_>>();

    let mut all = vec![];

    let mut sum = 0;
    for i in (0..input.len()).step_by(3) {
        let mut lhs = input[i].chars().collect();
        let mut rhs = input[i + 1].chars().collect();

        let lhs = build(&mut lhs);
        let rhs = build(&mut rhs);

        all.push(lhs.clone());
        all.push(rhs.clone());

        if lhs.cmp(&rhs).is_lt() {
            sum += (i / 3) + 1;
        }
    }

    let d1 = PacketElement::List(vec![PacketElement::List(vec![PacketElement::Integer(2)])]);
    all.push(d1.clone());

    let d2 = PacketElement::List(vec![PacketElement::List(vec![PacketElement::Integer(6)])]);
    all.push(d2.clone());
    all.sort();

    let p1 = all.iter().position(|e| e == &d1).unwrap();
    let p2 = all.iter().position(|e| e == &d2).unwrap();
    // println!("{all:?}");

    println!("{sum}");
    println!("{}", (p1 + 1) * (p2 + 1));
}
