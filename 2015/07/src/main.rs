use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn get_value<'a>(
    data: &std::collections::HashMap<&str, &'a str>,
    key: &'a str,
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    if let Some(int) = cache.get(key) {
        return *int;
    }
    if let Ok(int) = key.parse::<u16>() {
        return int;
    }

    let cmd = data[key];
    let parts = data[key].split(" ").collect::<Vec<&str>>();

    if cmd.contains("NOT") {
        let result = !get_value(&data, &parts[1], cache);
        return *cache.entry(key).or_insert(result);
    } else if cmd.contains("AND") {
        let result = get_value(&data, &parts[0], cache) & get_value(&data, &parts[2], cache);
        return *cache.entry(key).or_insert(result);
    } else if cmd.contains("OR") {
        let result = get_value(&data, &parts[0], cache) | get_value(&data, &parts[2], cache);
        return *cache.entry(key).or_insert(result);
    } else if cmd.contains("LSHIFT") {
        let result = get_value(&data, &parts[0], cache) << get_value(&data, &parts[2], cache);
        return *cache.entry(key).or_insert(result);
    } else if cmd.contains("RSHIFT") {
        let result = get_value(&data, &parts[0], cache) >> get_value(&data, &parts[2], cache);
        return *cache.entry(key).or_insert(result);
    } else {
        let result = get_value(&data, &parts[0], cache);
        cache.entry(key).or_insert(result);
        return result;
    }
}

fn main() {
    let input = File::open("input").expect("input file to exist");
    let buffer = BufReader::new(input);
    let lines = buffer.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut instructions = lines
        .iter()
        .map(|l| {
            let parts = l.split(" -> ").collect::<Vec<_>>();
            let cmd = parts[0];
            let key = parts[1];
            (key.trim(), cmd)
        })
        .collect::<HashMap<_, _>>();
    let mut cache = HashMap::new();
    let x = get_value(&instructions, "a", &mut cache);
    println!("part1: {:?}", x);

    let mut cache = HashMap::new();
    let new_signal = format!("{}", x);
    instructions.insert("b", &new_signal);
    let x = get_value(&instructions, "a", &mut cache);
    println!("part2: {:?}", x);
}
