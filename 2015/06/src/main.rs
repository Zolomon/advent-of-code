use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum Instruction {
    Toggle,
    On,
    Off,
}

struct Operation {
    i: Instruction,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn parse(line: &str, pattern: &Regex) -> Operation {
    let captures = pattern.captures(&line).unwrap();
    let i = match captures.name("instruction").unwrap().as_str() {
        "turn on" => Instruction::On,
        "turn off" => Instruction::Off,
        "toggle" => Instruction::Toggle,
        _ => panic!("Unknown"),
    };
    let p = |s| {
        captures
            .name(s)
            .unwrap()
            .as_str()
            .trim()
            .parse::<usize>()
            .unwrap()
    };

    let x1 = p("x1");
    let y1 = p("y1");
    let x2 = p("x2");
    let y2 = p("y2");

    Operation { i, x1, y1, x2, y2 }
}

fn eval(
    lights: &mut [i64; 1000000],
    instructions: &Vec<Operation>,
    f: fn(&mut i64, &Operation) -> (),
) {
    for i in instructions {
        for yy in i.y1..i.y2 + 1 {
            for xx in i.x1..i.x2 + 1 {
                let index = xx + (yy * W);
                f(&mut lights[index], &i);
            }
        }
    }
}

fn part1(value: &mut i64, i: &Operation) {
    *value = match i.i {
        Instruction::On => 1,
        Instruction::Off => 0,
        Instruction::Toggle => {
            if *value == 0 {
                1
            } else {
                0
            }
        }
    };
}

fn part2(value: &mut i64, i: &Operation) {
    *value += match &i.i {
        Instruction::On => 1,
        Instruction::Off => -1,
        Instruction::Toggle => 2,
    };
    if *value < 0 {
        *value = 0;
    }
}

const W: usize = 1000;
const H: usize = 1000;

fn main() {
    let input = File::open("input").expect("input file to exist");
    let buffer = BufReader::new(input);
    let lines = buffer.lines().map(|line| line.unwrap()).collect::<Vec<_>>();
    let pattern =
        Regex::new(r"(?P<instruction>.*?) (?P<x1>\d+),(?P<y1>\d+) through (?P<x2>\d+),(?P<y2>\d+)")
            .unwrap();
    let instructions = lines
        .iter()
        .map(|line| parse(&line, &pattern))
        .collect::<Vec<_>>();
    let mut lights = [0 as i64; W * H];
    eval(&mut lights, &instructions, part1);
    let part1 = lights.iter().filter(|x| **x == 1).count();

    lights.iter_mut().for_each(|x| *x = 0);
    eval(&mut lights, &instructions, part2);
    let part2: i64 = lights.iter().sum();

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
