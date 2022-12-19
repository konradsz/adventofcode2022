use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, PartialEq, Eq, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    minute: u32,

    ore_per_minute: u32,
    clay_per_minute: u32,
    obsidian_per_minute: u32,
    geode_per_minute: u32,

    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl State {
    fn new() -> Self {
        Self {
            minute: 0,
            ore_per_minute: 1,
            clay_per_minute: 0,
            obsidian_per_minute: 0,
            geode_per_minute: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn collect_materials(self) -> Self {
        Self {
            ore: self.ore + self.ore_per_minute,
            clay: self.clay + self.clay_per_minute,
            obsidian: self.obsidian + self.obsidian_per_minute,
            geode: self.geode + self.geode_per_minute,
            ..self
        }
    }

    fn mine(self) -> Self {
        let new_state = self.collect_materials();
        Self {
            minute: new_state.minute + 1,
            ..new_state
        }
    }

    fn construct_robot(self, materials_needed: (u32, u32, u32)) -> Option<Self> {
        if self.ore >= materials_needed.0
            && self.clay >= materials_needed.1
            && self.obsidian >= materials_needed.2
        {
            let mut new_state = self.collect_materials();
            new_state.ore -= materials_needed.0;
            new_state.clay -= materials_needed.1;
            new_state.obsidian -= materials_needed.2;

            Some(Self {
                minute: new_state.minute + 1,
                ..new_state
            })
        } else {
            None
        }
    }

    fn construct_ore_robot(self, blueprint: &HashMap<Material, (u32, u32, u32)>) -> Option<Self> {
        let materials_needed = blueprint.get(&Material::Ore).cloned().unwrap();
        self.construct_robot(materials_needed).map(|s| State {
            ore_per_minute: s.ore_per_minute + 1,
            ..s
        })
    }

    fn construct_clay_robot(self, blueprint: &HashMap<Material, (u32, u32, u32)>) -> Option<Self> {
        let materials_needed = blueprint.get(&Material::Clay).cloned().unwrap();
        self.construct_robot(materials_needed).map(|s| State {
            clay_per_minute: s.clay_per_minute + 1,
            ..s
        })
    }

    fn construct_obsidian_robot(
        self,
        blueprint: &HashMap<Material, (u32, u32, u32)>,
    ) -> Option<Self> {
        let materials_needed = blueprint.get(&Material::Obsidian).cloned().unwrap();
        self.construct_robot(materials_needed).map(|s| State {
            obsidian_per_minute: s.obsidian_per_minute + 1,
            ..s
        })
    }

    fn construct_geode_robot(self, blueprint: &HashMap<Material, (u32, u32, u32)>) -> Option<Self> {
        let materials_needed = blueprint.get(&Material::Geode).cloned().unwrap();
        self.construct_robot(materials_needed).map(|s| State {
            geode_per_minute: s.geode_per_minute + 1,
            ..s
        })
    }
}

fn parse_input_line(line: &str) -> HashMap<Material, (u32, u32, u32)> {
    let (_, rest) = line.split_once(':').unwrap();
    let mut segments = rest.trim().split_terminator('.');

    let ore_robot_cost: u32 = segments
        .next()
        .unwrap()
        .trim()
        .trim_start_matches("Each ore robot costs ")
        .trim_end_matches(" ore")
        .parse()
        .unwrap();

    let clay_robot_cost: u32 = segments
        .next()
        .unwrap()
        .trim()
        .trim_start_matches("Each clay robot costs ")
        .trim_end_matches(" ore")
        .parse()
        .unwrap();

    let (obsidian_robot_ore_cost, obsidian_robot_clay_cost): (u32, u32) = {
        let (ore, clay) = segments
            .next()
            .unwrap()
            .trim()
            .trim_start_matches("Each obsidian robot costs ")
            .trim_end_matches(" clay")
            .split_once(" ore and ")
            .unwrap();
        (ore.parse().unwrap(), clay.parse().unwrap())
    };

    let (geode_robot_ore_cost, geode_robot_obsidian_cost): (u32, u32) = {
        let (ore, clay) = segments
            .next()
            .unwrap()
            .trim()
            .trim_start_matches("Each geode robot costs ")
            .trim_end_matches(" obsidian")
            .split_once(" ore and ")
            .unwrap();
        (ore.parse().unwrap(), clay.parse().unwrap())
    };
    HashMap::from([
        (Material::Ore, (ore_robot_cost, 0, 0)),
        (Material::Clay, (clay_robot_cost, 0, 0)),
        (
            Material::Obsidian,
            (obsidian_robot_ore_cost, obsidian_robot_clay_cost, 0),
        ),
        (
            Material::Geode,
            (geode_robot_ore_cost, 0, geode_robot_obsidian_cost),
        ),
    ])
}

fn part_1(blueprints: &[HashMap<Material, (u32, u32, u32)>]) -> u32 {
    blueprints
        .iter()
        .enumerate()
        .map(|(idx, blueprint)| (idx as u32 + 1) * count_geodes(blueprint, 24))
        .sum()
}

fn part_2(blueprints: &[HashMap<Material, (u32, u32, u32)>]) -> u32 {
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| count_geodes(blueprint, 32))
        .product()
}

fn count_geodes(blueprint: &HashMap<Material, (u32, u32, u32)>, minutes: u32) -> u32 {
    let state = State::new();

    let mut states = VecDeque::new();
    states.push_back(state);

    let mut max = u32::MIN;
    let mut best_geodes = HashMap::new();
    let mut seen_states = HashSet::new();
    while let Some(state) = states.pop_front() {
        let mut state_to_cache = state.clone();
        state_to_cache.minute = 0;
        if !seen_states.insert(state_to_cache) {
            continue;
        }
        if state.minute == minutes {
            max = std::cmp::max(max, state.geode);
            continue;
        }

        let entry = best_geodes.entry(state.minute).or_insert(0);
        if *entry > state.geode {
            continue;
        } else {
            *entry = state.geode;
        }

        if let Some(s) = state.clone().construct_geode_robot(blueprint) {
            states.push_back(s);
        } else {
            if let Some(s) = state.clone().construct_ore_robot(blueprint) {
                states.push_back(s);
            }

            if let Some(s) = state.clone().construct_clay_robot(blueprint) {
                states.push_back(s);
            }

            if let Some(s) = state.clone().construct_obsidian_robot(blueprint) {
                states.push_back(s);
            }

            states.push_back(state.clone().mine());
        }
    }

    max
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let blueprints = input.lines().map(parse_input_line).collect::<Vec<_>>();

    // assert_eq!(part_1(&blueprints), 1023);
    assert_eq!(part_2(&blueprints), 13520);
}
