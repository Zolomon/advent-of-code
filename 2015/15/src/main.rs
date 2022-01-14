use regex::Regex;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[macro_use]
extern crate lazy_static;

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse(line: &String) -> Ingredient {
    lazy_static! {
        //Frosting: capacity 4, durability -2, flavor 0, texture 0, calories 5
        static ref RE: Regex = Regex::new(r".*?: capacity (?P<capacity>-?\d+), durability (?P<durability>-?\d+), flavor (?P<flavor>-?\d+), texture (?P<texture>-?\d+), calories (?P<calories>-?\d+)").unwrap();
    }
    let captures = RE.captures(line).unwrap();
    let parse_int = |captures: &regex::Captures, name| {
        captures
            .name(name)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .unwrap()
    };
    let capacity = parse_int(&captures, "capacity");
    let durability = parse_int(&captures, "durability");
    let flavor = parse_int(&captures, "flavor");
    let texture = parse_int(&captures, "texture");
    let calories = parse_int(&captures, "calories");
    Ingredient {
        capacity,
        durability,
        flavor,
        texture,
        calories,
    }
}

fn calculate_properties(ingredients: &Vec<Ingredient>) -> Vec<(i32, i32, i32, i32, i32)> {
    // TODO: Should not rely on knowledge of input fomrat, here we know there are only 4 ingredients. 😅
    itertools::iproduct!(0..101, 0..101, 0..101, 0..101)
        .filter(|(i, j, k, l)| i + j + k + l == 100)
        .map(|(i, j, k, l)| vec![i, j, k, l])
        .map(|x| {
            let mut capacity = 0;
            let mut durability = 0;
            let mut flavor = 0;
            let mut texture = 0;
            let mut calories = 0;
            for i in 0..ingredients.len() {
                capacity += x[i] * ingredients[i].capacity;
                durability += x[i] * ingredients[i].durability;
                flavor += x[i] * ingredients[i].flavor;
                texture += x[i] * ingredients[i].texture;
                calories += x[i] * ingredients[i].calories;
            }
            capacity = num::clamp(capacity, 0, std::i32::MAX);
            durability = num::clamp(durability, 0, std::i32::MAX);
            flavor = num::clamp(flavor, 0, std::i32::MAX);
            texture = num::clamp(texture, 0, std::i32::MAX);
            calories = num::clamp(calories, 0, std::i32::MAX);
            (capacity, durability, flavor, texture, calories)
        })
        .collect::<Vec<_>>()
}

fn main() {
    let input = File::open("input").expect("input file to exist");
    let buffer = BufReader::new(input);
    let lines = buffer.lines().map(|x| x.unwrap()).collect::<Vec<_>>();
    let ingredients = lines.iter().map(parse).collect::<Vec<_>>();
    let properties = calculate_properties(&ingredients);
    let part1 = properties
        .iter()
        .map(|(capacity, durability, flavor, texture, _)| {
            *capacity as i64 * *durability as i64 * *flavor as i64 * *texture as i64
        })
        .max()
        .unwrap();

    let part2 = properties
        .iter()
        .filter(|(_, _, _, _, calories)| *calories == 500)
        .map(|(capacity, durability, flavor, texture, _)| {
            *capacity as i64 * *durability as i64 * *flavor as i64 * *texture as i64
        })
        .max()
        .unwrap();

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
