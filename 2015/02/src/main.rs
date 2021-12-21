use std::cmp::{min, min_by};
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Gift {
    w: i64,
    h: i64,
    l: i64,
}

fn shortest(g: &Gift) -> std::option::Option<(i64, i64)> {
    vec![(g.w, g.h), (g.l, g.w), (g.l, g.h)]
        .iter()
        .cloned()
        .min_by(|x, y| (x.0 * x.1).cmp(&(y.0 * y.1)))
}

fn parse(line: String) -> Gift {
    let parts: Vec<_> = line.split("x").map(|x| x.parse::<i64>().unwrap()).collect();

    if parts.len() != 3 {
        panic!("line '{}' is malformed", line);
    }

    Gift {
        h: parts[0],
        w: parts[1],
        l: parts[2],
    }
}

fn surface_area(gift: &Gift) -> i64 {
    let extra = min(gift.h * gift.l, min(gift.w * gift.h, gift.w * gift.l));
    (2 * gift.l * gift.w) + (2 * gift.w * gift.h) + (2 * gift.h * gift.l) + extra
}

fn ribbon(gift: &Gift) -> i64 {
    let side = shortest(gift).unwrap();
    let ribbon = side.0 * 2 + side.1 * 2;
    let bow = gift.w * gift.w * gift.l;
    ribbon + bow
}

fn main() {
    let input = File::open("input").expect("No input file");
    let reader = BufReader::new(input);
    let lines: Vec<_> = reader.lines().map(|l| parse(l.unwrap())).collect();
    let part1 = lines.iter().fold(0, |acc, x| acc + surface_area(x));
    let part2 = lines.iter().fold(0, |acc, x| acc + ribbon(x));
    println!("Part 1: {}", part1);
    println!("Part 1: {}", part2);
}
