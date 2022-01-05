use serde_json::Value;
use std::fs::read_to_string;

fn rec(data: &Value, ignore_red: bool) -> i64 {
    match data {
        Value::Number(ref n) => n.as_i64().unwrap(),
        Value::Array(ref xs) => xs.iter().fold(0, |mut acc, x| {
            acc += rec(x, ignore_red);
            acc
        }),
        Value::Object(ref o) => {
            if ignore_red
                && o.values().any(|x| match x {
                    Value::String(ref s) => s == "red",
                    _ => false,
                })
            {
                return 0;
            }
            o.values().fold(0, |mut acc, x| {
                acc += rec(x, ignore_red);
                acc
            })
        }
        _ => 0,
    }
}

fn main() {
    let json = read_to_string("input").expect("input file to exist");
    let data: Value = serde_json::from_str(&json).unwrap();

    println!("part1: {}", rec(&data, false));
    println!("part2: {}", rec(&data, true));
}
