use itertools::Itertools;
use regex::{Regex, Captures};
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let input = File::open("input").expect("input file to exist");
    let buffer = BufReader::new(input);
    let lines = buffer.lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let string = lines.last().unwrap().clone();
    let line_count = lines.len() - 2;
    let parsed_values = &lines
        .clone()
        .into_iter()
        .take(line_count)
        .map(|x: std::string::String| {
            let parts = x.split(" => ").collect::<Vec<_>>();
            (parts[0].to_owned(), parts[1].to_owned())
        })
        .group_by(|(k, _)| k.clone());

    let mut groups = HashMap::<String, Vec<String>>::new();
    let mut rules = HashMap::<String, String>::new();
    for (k, v) in parsed_values.into_iter() {
        groups
            .entry(k)
            .or_insert(v.map(|x| x.1).collect::<Vec<_>>());
    }

    for (k, v) in lines
        .clone()
        .into_iter()
        .take(line_count)
        .map(|x: std::string::String| {
            let parts = x.split(" => ").collect::<Vec<_>>();
            (parts[0].to_owned(), parts[1].to_owned())
        })
        .group_by(|(k, _)| k.clone())
        .into_iter()
    {
        for key in v.map(|x| x.1).collect::<Vec<_>>() {
            rules.entry(key.chars().rev().collect::<String>())
                .or_insert(k.clone().chars().rev().collect());
        }
    }

    let mut distinct_molecules = HashSet::<String>::new();
    for i in 0..string.len() {
        for len in 1..3 {
            if i + len > string.len() {
                continue;
            }
            let c = &string[i..i + len];
            let key = c.to_owned();
            if groups.contains_key(&key) {
                for v in groups.get(&key).unwrap() {
                    let molecule =
                        format!("{}{}{}", &string[0..i], v, &string[i + len..string.len()]);
                    
                    distinct_molecules.insert(molecule);
                }
            }
        }
    }
    let part1 = distinct_molecules.len();

    let mut molecule = string.chars().rev().collect::<String>();
    let pattern = Regex::new(&rules.keys().join("|")).unwrap();
    let mut part2 = 0;
    while !molecule.eq("e") {
        molecule = pattern.replace(&molecule, |caps: &Captures| {
            rules[&caps[0]].clone()
        }).to_string();
        part2 += 1;
    }

    println!("part1: {part1}");
    println!("part2: {part2}");
}
