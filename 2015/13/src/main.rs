use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
enum Happiness {
    None,
    Left,
    Right,
}

fn generate_names<'a>(
    input: &Vec<String>,
    names: &'a mut HashSet<String>,
    costs: &mut HashMap<String, (Happiness, Option<i32>, Option<i32>)>,
) {
    let re = Regex::new(r"(?P<lhs>[a-zA-Z]+) would (?P<sign>(gain|lose)) (?P<cost>\d+) happiness units by sitting next to (?P<rhs>[a-zA-Z]+).").unwrap();
    for line in input {
        let captures = re.captures(&line).unwrap();
        let lhs = captures.name("lhs").unwrap().as_str();
        let sign = if captures.name("sign").unwrap().as_str() == "gain" {
            1
        } else {
            -1
        };
        let cost = sign
            * captures
                .name("cost")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
        let rhs = captures.name("rhs").unwrap().as_str();

        names.insert(lhs.to_string());
        names.insert(rhs.to_string());

        let to_from = format!("{} {}", lhs, rhs);
        let from_to = format!("{} {}", rhs, lhs);
        let x = costs.get(&to_from);

        if let None = x {
            costs.insert(to_from, (Happiness::Left, Some(cost), None));
        } else if let Some((Happiness::Left, None, Some(_))) = x {
            costs
                .entry(to_from)
                .and_modify(|mut e| {
                    (*e).0 = Happiness::Right;
                    (*e).1 = Some(cost);
                })
                .or_insert((Happiness::None, None, None));
        }

        let x = costs.get(&from_to);
        if let None = x {
            costs.insert(from_to, (Happiness::Left, None, Some(cost)));
        } else if let Some((Happiness::Left, Some(_), None)) = x {
            costs
                .entry(from_to)
                .and_modify(|mut e| {
                    (*e).0 = Happiness::Right;
                    (*e).2 = Some(cost);
                })
                .or_insert((Happiness::None, None, None));
        }
    }
}

fn arrange_optimal_seating(
    names: &mut HashSet<String>,
    costs: &mut HashMap<String, (Happiness, Option<i32>, Option<i32>)>,
) -> (std::vec::Vec<std::string::String>, i32) {
    let permutations = &mut names
        .iter()
        .permutations(names.len())
        .map(|path| {
            let parts = path
                .iter()
                .tuple_windows::<(_, _)>()
                .into_iter()
                .collect::<Vec<_>>();

            let mut sum = parts
                .iter()
                .map(|(from, to)| {
                    if let (Happiness::Right, Some(lhs), Some(rhs)) =
                        costs.get(format!("{} {}", from, to).as_str()).unwrap()
                    {
                        return lhs + rhs;
                    }
                    panic!("Should never happen: {} {}", from, to)
                })
                .sum::<i32>();

            let first = parts[0].0;
            let last = parts[parts.len() - 1].1;
            let value = costs.get(format!("{} {}", last, first).as_str()).unwrap();

            sum += value.1.unwrap() + value.2.unwrap();

            (path.iter().map(|&x| x.clone()).collect::<Vec<_>>(), sum)
        })
        .collect::<Vec<_>>();
    let (vec, i) = permutations.iter().max_by_key(|x| x.1).unwrap();
    (vec.to_vec(), i.clone())
}

fn main() {
    let input = File::open("input").expect("input file to exist");
    let buffer = BufReader::new(input);
    let lines = buffer.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut names = HashSet::<String>::new();
    let mut costs = HashMap::<String, (Happiness, Option<i32>, Option<i32>)>::new();
    generate_names(&lines, &mut names, &mut costs);

    let part1 = arrange_optimal_seating(&mut names, &mut costs);

    let my_name = "bengt";
    names.insert(my_name.to_owned());
    for name in &names {
        costs.insert(
            format!("{} {}", my_name, name),
            (Happiness::Right, Some(0), Some(0)),
        );
        costs.insert(
            format!("{} {}", name, my_name),
            (Happiness::Right, Some(0), Some(0)),
        );
    }

    let part2 = arrange_optimal_seating(&mut names, &mut costs);

    println!(
        "part1: permutations: {}, weight: {}",
        part1.0.iter().join(" "),
        part1.1
    );

    println!(
        "part2: permutations: {}, weight: {}",
        part2.0.iter().join(" "),
        part2.1
    );
}
