use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn calc(containers: &Vec<i32>, n: i32) -> (i32, HashMap<i32, i32>) {
    let mut total = 0;
    let mut groupings = HashMap::<i32, i32>::new();
    for i in 0..containers.len() {
        for c in containers.into_iter().combinations(i) {
            if c.iter().map(|x| **x).sum::<i32>() == n {
                total += 1;
                groupings
                    .entry(c.len() as i32)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
            }
        }
    }
    (total, groupings)
}

fn main() {
    let input = File::open("input").expect("input file to exist");
    let buffer = BufReader::new(input);
    let mut containers = buffer
        .lines()
        .map(|x| x.unwrap().as_str().parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    containers.sort();
    let n = 150;
    let (count, groupings) = calc(&containers, n);
    let smallest_group = groupings.iter().min_by_key(|(_, &v)| v).unwrap();

    let (part1, part2) = (count, smallest_group.1);
    println!("part1: {part1}");
    println!("part2: {part2}");
}
