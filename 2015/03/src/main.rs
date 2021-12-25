use num::Integer;
use std::collections::HashMap;
use std::fs::read_to_string;
use std::str::Chars;

fn count_visited_houses<'a, I>(instructions: I, visited: &mut HashMap<(i32, i32), i32>)
where
    I: Iterator<Item = char>,
{
    let deltas = HashMap::from([('>', (1, 0)), ('v', (0, -1)), ('<', (-1, 0)), ('^', (0, 1))]);
    instructions.map(|x| deltas[&x]).fold((0, 0), |acc, x| {
        let (xx, yy) = acc;
        let house = (xx + x.0, yy + x.1);
        *visited.entry(house).or_insert(0) += 1;
        house
    });
}

fn count(visited: &HashMap<(i32, i32), i32>)  -> usize {
    visited.iter().filter(|x| *x.1 > 0).count()
}

fn main() {
    let text = read_to_string("input").expect("No input file");
    
    let mut visited1 = HashMap::new();
    count_visited_houses(text.chars().map(|x| x), &mut visited1);
    
    let mut visited2 = HashMap::new();
    count_visited_houses(
        text.chars()
            .enumerate()
            .filter(|x| x.0 % 2 == 0)
            .map(|x| x.1),
            &mut visited2
    );
    count_visited_houses(
        text.chars()
            .enumerate()
            .filter(|x| x.0 % 2 == 1)
            .map(|x| x.1),
            &mut visited2
    );
    
    let part1 = count(&visited1);
    let part2 = count(&visited2);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
