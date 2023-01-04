use std::{
    collections::{BTreeSet, HashMap, VecDeque},
    hash::Hash,
};

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    tunnels: Vec<String>,
}

#[derive(Clone, Debug, Eq)]
struct State {
    minute: u32,
    current_valve: String,
    opened_valves: BTreeSet<String>,
    released_pressure: u32,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.minute == other.minute
            && self.current_valve == other.current_valve
            && self.opened_valves == other.opened_valves
    }
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.minute.hash(state);
        self.current_valve.hash(state);
        self.opened_valves.hash(state);
    }
}

fn main() {
    // let input = "Valve YK has flow rate=0; tunnels lead to valves GL, FT";
    // let (_, after) = input.split_at(6);
    // let (valve, after) = after.split_at(2);
    // let (_, after) = after.split_once('=').unwrap();
    // let (flow_rate, after) = after.split_once(';').unwrap();
    // let (_, tunnels) = after.split_at(24);
    // println!("{valve}, {flow_rate}, {tunnels}");

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
    // println!("{vulcano:?}");
    let mut states: HashMap<State, u32> = HashMap::new();
    // let vulcano = create_vulcano();

    states.insert(
        State {
            minute: 0,
            current_valve: "AA".into(),
            opened_valves: BTreeSet::new(),
            released_pressure: 0,
        },
        0,
    );

    let mut next_states = VecDeque::new();
    next_states.push_back(State {
        minute: 0,
        current_valve: "AA".into(),
        opened_valves: BTreeSet::new(),
        released_pressure: 0,
    });

    let mut max_released = u32::MIN;
    while let Some(state) = next_states.pop_front() {
        if state.minute == 30 {
            if state.released_pressure > max_released {
                max_released = state.released_pressure;
                println!("{max_released}, {}", state.opened_valves.len());
            }
            continue;
        }

        if !state.opened_valves.contains(&state.current_valve)
            && vulcano.get(&state.current_valve).unwrap().flow_rate != 0
        {
            let mut opened = state.opened_valves.clone();
            let released_pressure = calculate_released_pressure(&opened, &vulcano);
            opened.insert(state.current_valve.clone());
            let opened_valve_state = State {
                minute: state.minute + 1,
                current_valve: state.current_valve.clone(),
                opened_valves: opened,
                released_pressure: state.released_pressure + released_pressure,
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

        let tunnels = &vulcano.get(&state.current_valve).unwrap().tunnels;
        for tunnel in tunnels {
            let released_pressure = calculate_released_pressure(&state.opened_valves, &vulcano);
            let follow_tunnel_state = State {
                minute: state.minute + 1,
                current_valve: tunnel.clone(),
                opened_valves: state.opened_valves.clone(),
                released_pressure: state.released_pressure + released_pressure,
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

fn calculate_released_pressure(
    opened_valve: &BTreeSet<String>,
    vulcano: &HashMap<String, Valve>,
) -> u32 {
    opened_valve
        .iter()
        .map(|v| vulcano.get(v).unwrap().flow_rate)
        .sum()
}
