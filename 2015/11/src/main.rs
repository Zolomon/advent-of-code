fn main() {
    let mut input = String::from("cqjxjnds");

    let mut password1 = next_password(&mut input);
    let password2 = next_password(&mut password1);
    println!("part1: {}", password1);
    println!("part2: {}", password2);
}

fn next_password(input: &mut String) -> String {
    loop {
        let password = generate_password(&input);
        if is_valid(&password) {
            return password;
        }
        *input = password;
    }
}

fn is_valid(password: &str) -> bool {
    let mut one_c = password.chars().next().unwrap();
    let mut two_c = password.chars().skip(1).next().unwrap();
    if one_c == 'i' || one_c == 'o' || one_c == 'l' {
        return false;
    }
    if two_c == 'i' || two_c == 'o' || two_c == 'l' {
        return false;
    }
    let mut straight = false;
    let mut pairs = false;
    let mut pair_idxs = vec![];
    if one_c == two_c {
        pair_idxs.push(1);
    }

    for (i, c) in password.chars().skip(2).enumerate() {
        let i = i + 2;
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }

        if !straight && ((one_c as u8 + 1) as char == two_c && (one_c as u8 + 2) as char == c) {
            straight = true;
        }

        if !pairs {
            if two_c == c {
                pair_idxs.push(i);
            }
            if pair_idxs.len() >= 2
                && pair_idxs
                    .chunks(2)
                    .filter(|x| x.len() == 2 && x[1] - x[0] >= 2)
                    .count()
                    >= 1
            {
                pairs = true;
            }
        }

        one_c = two_c;
        two_c = c;
    }
    straight && pairs
}

fn generate_password(input: &str) -> String {
    let mut input = input.chars().collect::<Vec<char>>();
    let mut i = (input.len() - 1) as isize;
    loop {
        let out_of_bounds = i as usize >= input.len(); // overflow wrap-around
        if out_of_bounds {
            break;
        }
        let old_char = input[i as usize];
        let new_char = (((old_char as u8 + 1 - ('a' as u8)) % 26) + 'a' as u8) as char;
        input[i as usize] = new_char;
        let carry = (old_char as u8 + 1 - ('a' as u8)) / 26;
        if carry == 1 {
            i -= 1;
        } else {
            break;
        }
    }

    return input.iter().collect::<String>();
}

#[test]
fn test_valid_password() {
    assert_eq!(is_valid("hijklmmn"), false);
    assert_eq!(is_valid("abbceffg"), false);
    assert_eq!(is_valid("abbcegjk"), false);
    assert_eq!(is_valid("abcdffaa"), true);
    assert_eq!(is_valid("ghjaabcc"), true);
    assert_eq!(is_valid("cqjxppqr"), false);
}

#[test]
fn test_wrap_around() {
    assert_eq!(generate_password("a"), "b");
    assert_eq!(generate_password("z"), "a");
    assert_eq!(generate_password("xx"), "xy");
    assert_eq!(generate_password("xy"), "xz");
    assert_eq!(generate_password("xz"), "ya"); // Truncates at the end! *phew*
    assert_eq!(generate_password("ya"), "yb");
    assert_eq!(generate_password("zzz"), "aaa");
}
