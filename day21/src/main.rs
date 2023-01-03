use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Value {
    Number(u64),
    Add(String, String),
    Substract(String, String),
    Multiply(String, String),
    Divide(String, String),
    Equals(String, String),
    ToGuess,
}

fn parse_line(line: &str) -> (String, Value) {
    let (name, value) = line.split_once(':').unwrap();
    if let Ok(n) = value.trim().parse::<u64>() {
        (name.into(), Value::Number(n))
        // if name != "humn" {
        //     (name.into(), Value::Number(n))
        // } else {
        //     (name.into(), Value::ToGuess)
        // }
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

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut values = input.lines().map(parse_line).collect::<HashMap<_, _>>();

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

    println!("{values:?}");
    // for el in values {
    //     println!("{el:?}");
    // }
}
