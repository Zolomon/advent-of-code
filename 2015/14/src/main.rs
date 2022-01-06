use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[macro_use]
extern crate lazy_static;

struct Raindeer {
    speed: u32,
    duration: u32,
    rest_at: u32,
}

fn parse(lines: &Vec<String>) -> Vec<Raindeer> {
    // Vixen can fly 8 km/s for 8 seconds, but then must rest for 53 seconds.
    lazy_static! {
       static ref re: Regex = Regex::new(r".*? can fly (?P<speed>\d+) km/s for (?P<duration>\d+) seconds, but then must rest for (?P<rest_at>\d+) seconds\.").unwrap();
    }
    let mut raindeers = vec![];
    let parse_int = |captures: &regex::Captures, name| {
        captures
            .name(name)
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap()
    };
    for line in lines {
        let captures = re.captures(&line).unwrap();
        let speed = parse_int(&captures, "speed");
        let duration = parse_int(&captures, "duration");
        let rest_at = parse_int(&captures, "rest_at");
        raindeers.push(Raindeer {
            speed,
            duration,
            rest_at,
        });
    }

    raindeers
}

fn main() {
    let input = File::open("input").expect("input file to exist");
    let buffer = BufReader::new(input);
    let lines = buffer.lines().map(|x| x.unwrap()).collect::<Vec<_>>();

    let raindeers = parse(&lines);

    let n = 2503;
    let fastest_raindeer = raindeers
        .iter()
        .map(|x| {
            (0..x.duration)
                .map(|_| x.speed)
                .chain((0..(x.rest_at)).map(|_| 0))
                .cycle()
                .take(n)
                .sum::<u32>()
        })
        .max()
        .unwrap();
    println!("part1: {}", fastest_raindeer);

    let race = raindeers
        .iter()
        .map(|x| {
            let mut total = 0;
            (0..x.duration)
                .map(|_| x.speed)
                .chain((0..(x.rest_at)).map(|_| 0))
                .cycle()
                .take(n)
                .map(|x| {
                    total += x;
                    total
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let winners = vec![0; race.len()];
    let race = (0..n)
        .map(|s| {
            let positions = race.iter().map(|x| x[s]).collect::<Vec<_>>();
            let max = positions.iter().max().unwrap();
            positions
                .iter()
                .map(|v| if v == max { 1 } else { 0 })
                .collect::<Vec<_>>()
        })
        .fold(winners, |mut acc, x| {
            for (i, v) in x.iter().enumerate() {
                acc[i] += v;
            }
            acc
        });
    println!("part2: {}", race.iter().max().unwrap());
}
