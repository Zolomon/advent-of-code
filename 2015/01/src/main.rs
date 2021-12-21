use std::fs::read_to_string;
use std::io::Error;

struct Santa {
    pos: i32,
    sum: i32,
}

fn main() -> Result<(), Error> {
    let input = read_to_string("input").expect("No input file");
    let santa = input.chars().fold(Santa { pos: 1, sum: 0 }, |acc, x| {
        let result = acc.sum
            + match x {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };

        if result == -1 {
            panic!("{}", acc.pos);
        }

        Santa {
            pos: acc.pos + 1,
            sum: result,
        }
    });
    println!("sum: {}", santa.sum);
    Ok(())
}
