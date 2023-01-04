use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, HashMap, VecDeque},
    hash::Hash,
};

const BEAM_WIDTH: usize = 1000;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    tunnels: Vec<String>,
}

#[derive(Clone, Debug, Eq)]
struct State {
    minute: u32,
    current_valve: String,
    elephant_current_valve: String,
    opened_valves: BTreeSet<String>,
    released_pressure: u32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.minute == other.minute
            && self.current_valve == other.current_valve
            && self.elephant_current_valve == other.elephant_current_valve
            && self.opened_valves == other.opened_valves
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.minute.hash(state);
        self.current_valve.hash(state);
        self.elephant_current_valve.hash(state);
        self.opened_valves.hash(state);
    }
}

fn part_2(vulcano: &HashMap<String, Valve>) -> u32 {
    let mut states: HashMap<State, u32> = HashMap::new();

    states.insert(
        State {
            minute: 0,
            current_valve: "AA".into(),
            elephant_current_valve: "AA".into(),
            opened_valves: BTreeSet::new(),
            released_pressure: 0,
        },
        0,
    );

    let mut next_states = VecDeque::new();
    next_states.push_back(State {
        minute: 0,
        current_valve: "AA".into(),
        elephant_current_valve: "AA".into(),
        opened_valves: BTreeSet::new(),
        released_pressure: 0,
    });

    let mut beam = BinaryHeap::new();
    let mut max_released = u32::MIN;
    while let Some(state) = next_states.pop_front() {
        if state.minute == 26 {
            max_released = std::cmp::max(max_released, state.released_pressure);
            continue;
        }

        let released_pressure = calculate_released_pressure(&state.opened_valves, &vulcano);
        let next_released = state.released_pressure + released_pressure;

        if beam.len() < BEAM_WIDTH {
            beam.push(Reverse(next_released));
        } else {
            let smallest = beam.peek().unwrap().0;
            if next_released > smallest {
                beam.pop();
                beam.push(Reverse(next_released));
            } else if next_released < smallest {
                continue;
            }
        }

        let my_tunnels = &vulcano.get(&state.current_valve).unwrap().tunnels;
        let elephant_tunnels = &vulcano.get(&state.elephant_current_valve).unwrap().tunnels;

        // I open, elephant goes
        if !state.opened_valves.contains(&state.current_valve)
            && vulcano.get(&state.current_valve).unwrap().flow_rate != 0
        {
            for elephant_tunnel in elephant_tunnels {
                let mut opened = state.opened_valves.clone();
                opened.insert(state.current_valve.clone());
                let opened_valve_state = State {
                    minute: state.minute + 1,
                    current_valve: state.current_valve.clone(),
                    elephant_current_valve: elephant_tunnel.clone(),
                    opened_valves: opened,
                    released_pressure: next_released,
                };

                if let Some(rel) = states.get(&opened_valve_state) {
                    if rel >= &opened_valve_state.released_pressure {
                        continue;
                    }
                }
                states.insert(
                    opened_valve_state.clone(),
                    opened_valve_state.released_pressure,
                );
                next_states.push_back(opened_valve_state);
            }
        }

        // I go, elephant opens
        if !state.opened_valves.contains(&state.elephant_current_valve)
            && vulcano
                .get(&state.elephant_current_valve)
                .unwrap()
                .flow_rate
                != 0
        {
            for my_tunnel in my_tunnels {
                let mut opened = state.opened_valves.clone();
                opened.insert(state.elephant_current_valve.clone());
                let opened_valve_state = State {
                    minute: state.minute + 1,
                    current_valve: my_tunnel.clone(),
                    elephant_current_valve: state.elephant_current_valve.clone(),
                    opened_valves: opened,
                    released_pressure: next_released,
                };

                if let Some(rel) = states.get(&opened_valve_state) {
                    if rel >= &opened_valve_state.released_pressure {
                        continue;
                    }
                }
                states.insert(
                    opened_valve_state.clone(),
                    opened_valve_state.released_pressure,
                );
                next_states.push_back(opened_valve_state);
            }
        }

        // we both open
        if state.elephant_current_valve != state.current_valve
            && !state.opened_valves.contains(&state.current_valve)
            && vulcano.get(&state.current_valve).unwrap().flow_rate != 0
            && !state.opened_valves.contains(&state.elephant_current_valve)
            && vulcano
                .get(&state.elephant_current_valve)
                .unwrap()
                .flow_rate
                != 0
        {
            let mut opened = state.opened_valves.clone();
            opened.insert(state.current_valve.clone());
            opened.insert(state.elephant_current_valve.clone());
            let opened_valve_state = State {
                minute: state.minute + 1,
                current_valve: state.current_valve.clone(),
                elephant_current_valve: state.elephant_current_valve.clone(),
                opened_valves: opened,
                released_pressure: next_released,
            };

            if let Some(rel) = states.get(&opened_valve_state) {
                if rel >= &opened_valve_state.released_pressure {
                    continue;
                }
            }
            states.insert(
                opened_valve_state.clone(),
                opened_valve_state.released_pressure,
            );
            next_states.push_back(opened_valve_state);
        }

        // we both move
        for my_tunnel in my_tunnels {
            for elephant_tunnel in elephant_tunnels {
                if my_tunnel == elephant_tunnel {
                    continue;
                }
                let follow_tunnel_state = State {
                    minute: state.minute + 1,
                    current_valve: my_tunnel.clone(),
                    elephant_current_valve: elephant_tunnel.clone(),
                    opened_valves: state.opened_valves.clone(),
                    released_pressure: next_released,
                };

                if let Some(rel) = states.get(&follow_tunnel_state) {
                    if rel >= &follow_tunnel_state.released_pressure {
                        continue;
                    }
                }
                states.insert(
                    follow_tunnel_state.clone(),
                    follow_tunnel_state.released_pressure,
                );
                next_states.push_back(follow_tunnel_state);
            }
        }
    }
    max_released
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let vulcano = input
        .lines()
        .map(|l| {
            let (_, after) = l.split_at(6);
            let (valve, after) = after.split_at(2);
            let (_, after) = after.split_once('=').unwrap();
            let (flow_rate, after) = after.split_once(';').unwrap();
            let (_, tunnels) = after.split_at(23);
            let tunnels = tunnels.trim();
            (
                valve.to_string(),
                Valve {
                    flow_rate: flow_rate.parse().unwrap(),
                    tunnels: tunnels.split(", ").map(|s| s.to_string()).collect(),
                },
            )
        })
        .collect::<HashMap<_, _>>();

    assert_eq!(part_2(&vulcano), 2723);
}

fn calculate_released_pressure(
    opened_valve: &BTreeSet<String>,
    vulcano: &HashMap<String, Valve>,
) -> u32 {
    opened_valve
        .iter()
        .map(|v| vulcano.get(v).unwrap().flow_rate)
        .sum()
}
