use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;

fn part1_rule(line: &String) -> bool {
    let vowels = HashSet::from(['a', 'e', 'i', 'o', 'u']); // TODO: Make static somehow
    let vowel_count = line.chars().filter(|c| vowels.contains(c)).count() >= 3;
    let two_in_a_row = line.chars().zip(line.chars().skip(1)).any(|x| x.0 == x.1);
    let invalid_pattern = Regex::new(r"ab|cd|pq|xy").unwrap(); // TODO: Make static somehow
    let invalid = invalid_pattern.is_match(&line);

    vowel_count && two_in_a_row && !invalid
}

fn part2_rule(line: &String) -> bool {
    let duplicates = line.as_bytes().windows(2).enumerate().any(|(i, c)| {
        line.rfind(std::str::from_utf8(c).unwrap())
            .map(|index| index > i + 1)
            .unwrap_or(false)
    });
    let triplets = line.as_bytes().windows(3).any(|x| x[0] == x[2]);
    duplicates && triplets
}

fn main() {
    let input = File::open("input").expect("No input file");
    let buffer = BufReader::new(input);
    let lines: Vec<String> = buffer.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let part1 = lines.iter().map(part1_rule).filter(|x| *x).count();
    let part2 = lines.iter().map(part2_rule).filter(|x| *x).count();
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
