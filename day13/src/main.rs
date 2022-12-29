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

fn parse_list(input: &mut VecDeque<char>) -> Vec<PacketElement> {
    let mut result = vec![];
    loop {
        let front = input.pop_front().unwrap();
        match front {
            '[' => result.push(PacketElement::List(parse_list(input))),
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

fn parse(input: &mut VecDeque<char>) -> PacketElement {
    input.pop_front();
    PacketElement::List(parse_list(input))
}

fn part_1(pairs: &[&str]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter_map(|(idx, pair)| {
            let (lhs, rhs) = pair.split_once('\n').unwrap();
            let lhs = parse(&mut lhs.chars().collect());
            let rhs = parse(&mut rhs.chars().collect());
            lhs.cmp(&rhs).is_lt().then_some(idx + 1)
        })
        .sum()
}

fn part_2(pairs: &[&str]) -> usize {
    let mut packets = vec![];
    for pair in pairs {
        let (lhs, rhs) = pair.split_once('\n').unwrap();
        let lhs = parse(&mut lhs.chars().collect());
        let rhs = parse(&mut rhs.chars().collect());
        packets.push(lhs);
        packets.push(rhs);
    }

    let divider1 = PacketElement::List(vec![PacketElement::List(vec![PacketElement::Integer(2)])]);
    packets.push(divider1.clone());

    let divider2 = PacketElement::List(vec![PacketElement::List(vec![PacketElement::Integer(6)])]);
    packets.push(divider2.clone());

    packets.sort();

    let p1 = packets.iter().position(|e| e == &divider1).unwrap();
    let p2 = packets.iter().position(|e| e == &divider2).unwrap();

    (p1 + 1) * (p2 + 1)
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let pairs = input.split("\n\n").collect::<Vec<_>>();

    assert_eq!(part_1(&pairs), 5717);
    assert_eq!(part_2(&pairs), 25935);
}
