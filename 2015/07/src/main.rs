use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// #[derive(Debug)]
// enum Instruction<'a> {
//     ValueInt(u16),
//     ValueVar(&'a str),
//     AndVarVar(&'a str, &'a str),
//     AndIntVar(u16, &'a str),
//     OrVarVar(&'a str, &'a str),
//     OrIntVar(u16, &'a str),
//     NotVar(&'a str),
//     NotInt(u16),
//     LeftShift(&'a str, u16),
//     RightShift(&'a str, u16),
// }

// fn parse(line: &String) -> (&str, Instruction) {
//     lazy_static! {
//         static ref VAL_REG: Regex = Regex::new(r"^(?P<value>([a-z]+|\d+)) -> (?P<variable>.*)$").unwrap();
//         static ref OP_REG: Regex = Regex::new(
//             r"^(?P<lhs>([a-z]+|\d+)) (?P<operator>(AND|OR|LSHIFT|RSHIFT)) (?P<rhs>([a-z]+|\d+)) -> (?P<variable>[a-z]+)$"
//         )
//         .unwrap();
//         static ref NOT_REG: Regex = Regex::new(r"^NOT (?P<value>([a-z]+|\d+)) -> (?P<variable>.*)$").unwrap();
//     }

//     if VAL_REG.is_match(line) {
//         println!("VALUE_REG: {}", line);
//         let captures = VAL_REG.captures(line).unwrap();
//         let value = captures.name("value").unwrap().as_str();
//         let variable = captures.name("variable").unwrap().as_str();
//         if let Ok(value) = value.parse::<u16>() {
//             return (variable, Instruction::ValueInt(value));
//         } else {
//             return (variable, Instruction::ValueVar(value));
//         }
//     } else if OP_REG.is_match(line) {
//         println!("OP_REG: {}", line);
//         let captures = OP_REG.captures(line).unwrap();
//         let lhs = captures.name("lhs").unwrap().as_str();
//         let rhs = captures.name("rhs").unwrap().as_str();
//         let variable = captures.name("variable").unwrap().as_str();
//         let with_int = |int| match captures.name("operator").unwrap().as_str() {
//             "AND" => (variable, Instruction::AndIntVar(int, rhs)),
//             "OR" => (variable, Instruction::OrIntVar(int, rhs)),
//             "LSHIFT" => (
//                 variable,
//                 Instruction::LeftShift(lhs, rhs.parse::<u16>().unwrap()),
//             ),
//             "RSHIFT" => (
//                 variable,
//                 Instruction::RightShift(lhs, rhs.parse::<u16>().unwrap()),
//             ),
//             _ => panic!(),
//         };
//         if let Ok(int) = lhs.parse::<u16>() {
//             return with_int(int);
//         } else if let Ok(int) = rhs.parse::<u16>() {
//             return with_int(int);
//         } else {
//             return match captures.name("operator").unwrap().as_str() {
//                 "AND" => (variable, Instruction::AndVarVar(lhs, rhs)),
//                 "OR" => (variable, Instruction::OrVarVar(lhs, rhs)),
//                 "LSHIFT" => (
//                     variable,
//                     Instruction::LeftShift(lhs, rhs.parse::<u16>().unwrap()),
//                 ),
//                 "RSHIFT" => (
//                     variable,
//                     Instruction::RightShift(lhs, rhs.parse::<u16>().unwrap()),
//                 ),
//                 _ => panic!(),
//             };
//         }
//     } else if VAL_REG.is_match(line) {
//         println!("VARIABLE_REG: {}", line);
//         let captures = VAL_REG.captures(line).unwrap();
//         let value = captures.name("value").unwrap().as_str();
//         let variable = captures.name("variable").unwrap().as_str();
//         if let Ok(value) = value.parse::<u16>() {
//             return (variable, Instruction::ValueInt(value));
//         } else {
//             return (variable, Instruction::ValueVar(value));
//         }
//     } else {
//         println!("NOT_REG: {}", line);
//         let captures = NOT_REG.captures(line).unwrap();
//         let value = captures.name("value").unwrap().as_str();
//         let variable = captures.name("variable").unwrap().as_str();
//         if let Ok(value) = value.parse::<u16>() {
//             return (variable, Instruction::NotInt(value));
//         } else {
//             return (variable, Instruction::NotVar(value));
//         }
//     }
// }

// fn eval(instructions: &HashMap<&str, Instruction>, var: &str) -> u16 {
//     eval_rec(&instructions, var, instructions.get(var).unwrap())
// }

// fn eval_rec(instructions: &HashMap<&str, Instruction>, v: &str, i: &Instruction) -> u16 {
//     println!("var: {}, i: {:?}", v, i);
//     return match i {
//         Instruction::ValueVar(var) => eval_rec(instructions, var, instructions.get(var).unwrap()),
//         Instruction::ValueInt(value) => *value,
//         Instruction::AndVarVar(lhs, rhs) => {
//             eval_rec(instructions, lhs, instructions.get(lhs).unwrap())
//                 & eval_rec(instructions, rhs, instructions.get(rhs).unwrap())
//         }
//         Instruction::AndIntVar(lhs, rhs) => {
//             lhs & eval_rec(instructions, rhs, instructions.get(rhs).unwrap())
//         }
//         Instruction::OrVarVar(lhs, rhs) => {
//             eval_rec(instructions, lhs, instructions.get(lhs).unwrap())
//                 | eval_rec(instructions, rhs, instructions.get(rhs).unwrap())
//         }
//         Instruction::OrIntVar(lhs, rhs) => {
//             lhs | eval_rec(instructions, rhs, instructions.get(rhs).unwrap())
//         }
//         Instruction::LeftShift(lhs, value) => {
//             eval_rec(instructions, lhs, instructions.get(lhs).unwrap()) << value
//         }
//         Instruction::RightShift(lhs, value) => {
//             eval_rec(instructions, lhs, instructions.get(lhs).unwrap()) >> value
//         }
//         Instruction::NotVar(var) => !eval_rec(instructions, var, instructions.get(var).unwrap()),
//         Instruction::NotInt(value) => !*value,
//     };
// }

fn get_value<'a>(
    data: &std::collections::HashMap<&str, &'a str>,
    key: &'a str,
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    if let Some(int) = cache.get(key) {
        println!("Found k:{}=v:{} in cache", key, int);
        return *int;
    }
    if let Ok(int) = key.parse::<u16>() {
        return int;
    }
    //cache.insert(key,int);
    let cmd = data[key];
    let parts = data[key].split(" ").collect::<Vec<&str>>();
    println!("key: {} cmd: {}", key, cmd);
    if cmd.contains("NOT") {
        let result = !get_value(&data, &parts[1], cache);
        return *cache.entry(key).or_insert(result);
    } else if cmd.contains("AND") {
        let result = get_value(&data, &parts[0], cache) & get_value(&data, &parts[2], cache);
        return *cache.entry(key).or_insert(result);
    } else if cmd.contains("OR") {
        let result = get_value(&data, &parts[0], cache) | get_value(&data, &parts[2], cache);
        return *cache.entry(key).or_insert(result);
    } else if cmd.contains("LSHIFT") {
        let result = get_value(&data, &parts[0], cache) << get_value(&data, &parts[2], cache);
        return *cache.entry(key).or_insert(result);
    } else if cmd.contains("RSHIFT") {
        let result = get_value(&data, &parts[0], cache) >> get_value(&data, &parts[2], cache);
        return *cache.entry(key).or_insert(result);
    } else {
        let result = get_value(&data, &parts[0], cache);
        cache.entry(key).or_insert(result);
        return result;
    }
}

fn main() {
    let input = File::open("input").expect("input file to exist");
    let buffer = BufReader::new(input);
    let lines = buffer.lines().map(|l| l.unwrap()).collect::<Vec<_>>();
    // let lines = [
    //     String::from("123 -> x"),
    //     String::from("456 -> y"),
    //     String::from("x AND y -> d"),
    //     String::from("x OR y -> e"),
    //     String::from("x LSHIFT 2 -> f"),
    //     String::from("y RSHIFT 2 -> g"),
    //     String::from("NOT x -> h"),
    //     String::from("NOT y -> i"),
    // ];

    let mut instructions = lines
        .iter()
        .map(|l| {
            let parts = l.split(" -> ").collect::<Vec<_>>();
            let cmd = parts[0];
            let key = parts[1];
            // println!("k:{} v:{}", key, cmd);
            (key.trim(), cmd)
        })
        .collect::<HashMap<_, _>>();
    let mut cache = HashMap::new();
    let x = get_value(&instructions, "a", &mut cache);
    println!("{:?}", x);
    // instructions.insert("a", "b");

    let mut cache = HashMap::new();
    let mut instructions = lines
        .iter()
        .map(|l| {
            let parts = l.split(" -> ").collect::<Vec<_>>();
            let cmd = parts[0];
            let key = parts[1];
            // println!("k:{} v:{}", key, cmd);
            (key.trim(), cmd)
        })
        .collect::<HashMap<_, _>>();
    let s = format!("{}", x);
    instructions.insert("b", &s);
    let x = get_value(&instructions, "a", &mut cache);
    println!("{:?}", x);
    // for x in &instructions {
    //     println!("var: {} inst: {:?}", x.0, x.1);
    // }

    // println!("EVAL:");
    // for x in &instructions {
    //     println!("var: {} value: {:?}\n", x.0, eval(&instructions, x.0));
    // }
    // for (k, v) in &instructions {
    //     println!("{} {:?}", k, v);
    // }
    //println!("{:?}", eval(&instructions, "a"));
}
