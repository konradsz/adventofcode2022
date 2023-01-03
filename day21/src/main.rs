use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Value {
    Number(u64),
    Add(String, String),
    Substract(String, String),
    Multiply(String, String),
    Divide(String, String),
    Equals(String, String),
}

fn parse_line(line: &str) -> (String, Value) {
    let (name, value) = line.split_once(':').unwrap();
    if let Ok(n) = value.trim().parse::<u64>() {
        (name.into(), Value::Number(n))
    } else {
        if value.contains('+') {
            let (lhs, rhs) = value.split_once('+').unwrap();
            (
                name.into(),
                Value::Add(lhs.trim().into(), rhs.trim().into()),
            )
        } else if value.contains('-') {
            let (lhs, rhs) = value.split_once('-').unwrap();
            (
                name.into(),
                Value::Substract(lhs.trim().into(), rhs.trim().into()),
            )
        } else if value.contains('*') {
            let (lhs, rhs) = value.split_once('*').unwrap();
            (
                name.into(),
                Value::Multiply(lhs.trim().into(), rhs.trim().into()),
            )
        } else {
            let (lhs, rhs) = value.split_once('/').unwrap();
            (
                name.into(),
                Value::Divide(lhs.trim().into(), rhs.trim().into()),
            )
        }
    }
}

fn part_1(mut values: HashMap<String, Value>) -> u64 {
    evaluate(&mut values);

    if let Value::Number(n) = values.get("root").unwrap() {
        return *n;
    } else {
        panic!()
    }
}

fn part_2(mut values: HashMap<String, Value>) -> u64 {
    values.remove("humn");
    let root = values.get_mut("root").unwrap();
    match root {
        Value::Add(lhs, rhs) => *root = Value::Equals(lhs.clone(), rhs.clone()),
        Value::Substract(lhs, rhs) => *root = Value::Equals(lhs.clone(), rhs.clone()),
        Value::Multiply(lhs, rhs) => *root = Value::Equals(lhs.clone(), rhs.clone()),
        Value::Divide(lhs, rhs) => *root = Value::Equals(lhs.clone(), rhs.clone()),
        Value::Equals(_, _) | Value::Number(_) => panic!(),
    }

    evaluate(&mut values);

    let known_numbers = values
        .iter()
        .filter_map(|(name, value)| {
            if let Value::Number(n) = value {
                Some((name.clone(), *n))
            } else {
                None
            }
        })
        .collect::<HashMap<_, _>>();

    let mut current = "root".to_owned();
    let mut result = 0;
    loop {
        if current == "humn" {
            break;
        }
        // println!("{current}");
        let val = values.get_mut(&current).unwrap();
        match val {
            Value::Equals(lhs, rhs) => {
                if let Some(n) = known_numbers.get(lhs) {
                    result = *n;
                    current = rhs.clone();
                }
                if let Some(n) = known_numbers.get(rhs) {
                    result = *n;
                    current = lhs.clone();
                }
            }
            Value::Add(lhs, rhs) => {
                if let Some(n) = known_numbers.get(rhs) {
                    result -= *n;
                    current = lhs.clone();
                }
                if let Some(n) = known_numbers.get(lhs) {
                    result -= *n;
                    current = rhs.clone();
                }
            }
            Value::Substract(lhs, rhs) => {
                if let Some(n) = known_numbers.get(rhs) {
                    result += *n;
                    current = lhs.clone();
                }
                if let Some(n) = known_numbers.get(lhs) {
                    result = *n - result;
                    current = rhs.clone();
                }
            }
            Value::Multiply(lhs, rhs) => {
                if let Some(n) = known_numbers.get(rhs) {
                    result /= *n;
                    current = lhs.clone();
                }
                if let Some(n) = known_numbers.get(lhs) {
                    result /= *n;
                    current = rhs.clone();
                }
            }
            Value::Divide(lhs, rhs) => {
                if let Some(n) = known_numbers.get(rhs) {
                    result *= *n;
                    current = lhs.clone();
                }
                if let Some(n) = known_numbers.get(lhs) {
                    result = n / result;
                    current = rhs.clone();
                }
            }
            Value::Number(_) => panic!(),
        }
    }
    result
}

fn evaluate(values: &mut HashMap<String, Value>) {
    let mut known = values
        .values()
        .filter(|v| matches!(v, Value::Number(_)))
        .count();
    while values.values().any(|v| !matches!(v, Value::Number(_))) {
        let old_values = values.clone();
        for v in values.values_mut() {
            match v {
                Value::Add(lhs, rhs) => {
                    if let Some(Value::Number(lhs)) = old_values.get(lhs) {
                        if let Some(Value::Number(rhs)) = old_values.get(rhs) {
                            *v = Value::Number(lhs + rhs);
                        }
                    }
                }
                Value::Substract(lhs, rhs) => {
                    if let Some(Value::Number(lhs)) = old_values.get(lhs) {
                        if let Some(Value::Number(rhs)) = old_values.get(rhs) {
                            *v = Value::Number(lhs - rhs);
                        }
                    }
                }
                Value::Multiply(lhs, rhs) => {
                    if let Some(Value::Number(lhs)) = old_values.get(lhs) {
                        if let Some(Value::Number(rhs)) = old_values.get(rhs) {
                            *v = Value::Number(lhs * rhs);
                        }
                    }
                }
                Value::Divide(lhs, rhs) => {
                    if let Some(Value::Number(lhs)) = old_values.get(lhs) {
                        if let Some(Value::Number(rhs)) = old_values.get(rhs) {
                            *v = Value::Number(lhs / rhs);
                        }
                    }
                }
                _ => (),
            }
        }
        let new_count = values
            .values()
            .filter(|v| matches!(v, Value::Number(_)))
            .count();
        if new_count == known {
            break;
        } else {
            known = new_count;
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let values = input.lines().map(parse_line).collect::<HashMap<_, _>>();

    assert_eq!(part_1(values.clone()), 63119856257960);
    assert_eq!(part_2(values), 3006709232464);
}
