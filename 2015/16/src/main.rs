use regex::Regex;
use regex::RegexSet;
use std::cmp::max;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[macro_use]
extern crate lazy_static;

#[derive(std::cmp::PartialEq)]
enum Part {
    One,
    Two,
}

fn parse(line: String) -> HashMap<&'static str, i32> {
    lazy_static! {
        static ref SET: Vec<(&'static str, Regex)> = vec![
            ("sue", Regex::new(r"Sue (?P<sue>\d+)").unwrap()),
            (
                "samoyeds",
                Regex::new(r"samoyeds: (?P<samoyeds>\d+)").unwrap()
            ),
            (
                "pomeranians",
                Regex::new(r"pomeranians: (?P<pomeranians>\d+)").unwrap()
            ),
            ("akitas", Regex::new(r"akitas: (?P<akitas>\d+)").unwrap()),
            ("vizslas", Regex::new(r"vizslas: (?P<vizslas>\d+)").unwrap()),
            (
                "perfumes",
                Regex::new(r"perfumes: (?P<perfumes>\d+)").unwrap()
            ),
            ("cats", Regex::new(r"cats: (?P<cats>\d+)").unwrap()),
            ("trees", Regex::new(r"trees: (?P<trees>\d+)").unwrap()),
            ("cars", Regex::new(r"cars: (?P<cars>\d+)").unwrap()),
            (
                "goldfish",
                Regex::new(r"goldfish: (?P<goldfish>\d+)").unwrap()
            ),
        ];
    }

    let captures = SET
        .iter()
        .filter(|(_, re)| !re.captures(&line).is_none())
        .map(|(name, re)| {
            (
                *name,
                re.captures(&line)
                    .unwrap()
                    .name(name)
                    .unwrap()
                    .as_str()
                    .parse::<i32>()
                    .unwrap(),
            )
        })
        .collect::<HashMap<_, i32>>();

    captures
}

fn find_sue(
    sue: &HashMap<&str, i32>,
    lines: &Vec<HashMap<&str, i32>>,
    part: Part,
) -> (Option<i32>, usize) {
    let mut id = None;
    let mut max_count = 0;
    let not_in = HashSet::from(["trees", "cats", "pomeranians", "goldfish"]);
    for line in lines {
        let mut l = line
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .filter(|(k, _)| *k != "sue")
            .collect::<HashSet<_>>();
        if part == Part::Two {
            l = l
                .iter()
                .map(|(k, v)| (*k, v.clone()))
                .filter(|(k, _)| !not_in.contains(k))
                .collect::<HashSet<_>>();
        }
        let s = sue
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect::<HashSet<_>>();

        let intersection = s.intersection(&l).collect::<HashSet<_>>();
        let count = intersection.len();
        if max_count < count {
            id = line.get("sue").copied();
            max_count = count;
        }
    }
    (id, max_count)
}

fn main() {
    let input = File::open("input").expect("input file to exist");
    let buffer = BufReader::new(input);
    let lines = buffer
        .lines()
        .map(|x| x.unwrap().replace(",", ""))
        .map(parse)
        .collect::<Vec<_>>();

    let sue = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
    let (part1, max_count) = find_sue(&sue, &lines, Part::One);
    let (part2, max_count) = find_sue(&sue, &lines, Part::Two);

    println!("part1: {}", part1.unwrap());
    println!("part2: {}", part2.unwrap());
}
