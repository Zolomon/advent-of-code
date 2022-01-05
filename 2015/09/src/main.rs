use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = File::open("input").expect("input file to exist");
    let buffer = BufReader::new(input);
    let lines = buffer.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut graph = HashMap::<String, u32>::new();

    let re = Regex::new(r"(?P<from>.*?) to (?P<to>.*?) = (?P<weight>\d+)").unwrap();
    let mut cities = HashSet::<&str>::new();
    for line in &lines {
        let capture = re.captures(&line).unwrap();
        let from = capture.name("from").unwrap().as_str();
        let to = capture.name("to").unwrap().as_str();
        let weight = capture
            .name("weight")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();
        let to_from = format!("{} {}", to, from);
        let from_to = format!("{} {}", from, to);
        graph.entry(to_from).or_insert(weight);
        graph.entry(from_to).or_insert(weight);
        cities.insert(from);
        cities.insert(to);
    }

    let permutations = cities
        .iter()
        .map(|city| *city)
        .permutations(cities.len())
        .map(|path| {
            let sum = path
                .iter()
                .tuple_windows::<(_, _)>()
                .into_iter()
                .map(|(from, to)| graph.get(format!("{} {}", from, to).as_str()).unwrap())
                .sum::<u32>();

            (path, sum)
        })
        .collect::<Vec<_>>();
    let part1 = permutations.iter().min_by_key(|x| x.1).unwrap();
    let part2 = permutations.iter().max_by_key(|x| x.1).unwrap();

    println!(
        "part1: permutations: {}, weight: {}",
        part1.0.join(" "),
        part1.1
    );
    println!(
        "part2: permutations: {}, weight: {}",
        part2.0.join(" "),
        part2.1
    );
}
