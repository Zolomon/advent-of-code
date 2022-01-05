fn look_and_say(input: &mut String, mut depth: usize) -> usize {
    while depth > 0 {
        // Fold chars into groups of consecutive numbers
        let result = input.chars().fold((vec![], None), |mut acc, x| {
            if acc.1 == None {
                acc.0.push(vec![x]);
                return (acc.0, Some(x));
            } else {
                if acc.1 == Some(x) {
                    let last = &mut acc.0.last_mut().unwrap();
                    last.push(x);
                } else {
                    acc.0.push(vec![x]);
                }
                return (acc.0, Some(x));
            }
        });
        *input = result
            .0
            .iter()
            .map(|x| format!("{}{}", x.len(), x.first().unwrap()))
            .collect::<Vec<_>>()
            .join("");

        depth -= 1;
    }
    return input.len();
}

fn main() {
    let input = String::from("1321131112");
    let part1 = look_and_say(&mut input.clone(), 40);
    let part2 = look_and_say(&mut input.clone(), 50);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}
