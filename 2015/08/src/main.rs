use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse(line: &String) -> (u32, u32) {
    let bytes = line.chars().collect::<Vec<char>>();
    let mut i = 0;
    let mut code = 0;
    let mut mem = 0;
    while i < bytes.len() {
        let byte = bytes[i];
        if byte == '"' {
            mem += 1;
        } else if byte == '\\' {
            if bytes[i + 1] == 'x' {
                mem += 4;
                code += 1;
                i += 3;
            } else {
                mem += 2;
                code += 1;
                i += 1;
            }
        } else if byte >= 'a' || byte <= 'z' {
            code += 1;
            mem += 1;
        }
        i += 1;
    }
    (mem, code)
}

fn encode(line: &String) -> String {
    let mut encoded = Vec::<char>::new();
    encoded.push('\"');
    for c in line.chars() {
        if c == '\"' {
            encoded.push('\\');
            encoded.push('\"');
        } else if c == '\\' {
            encoded.push('\\');
            encoded.push('\\');
        } else {
            encoded.push(c);
        }
    }
    encoded.push('\"');
    encoded.iter().collect::<String>()
}

fn main() {
    let input = File::open("input").expect("input file to exist");
    let buffer = BufReader::new(input);
    let lines = buffer.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    let encoded_lines = lines.clone().iter().map(encode).collect::<Vec<_>>();

    let parsed_lines = lines.iter().map(parse).collect::<Vec<_>>();
    let part1 = parsed_lines.iter().fold(0, |acc, x| {
        let (mem, code) = x;
        acc + (mem - code)
    });

    let part2 = encoded_lines
        .iter()
        .map(parse)
        .zip(parsed_lines.iter())
        .fold(0, |acc, x| {
            let ((mem2, _), (mem1, _)) = x;
            acc + (mem2 - mem1)
        });
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
