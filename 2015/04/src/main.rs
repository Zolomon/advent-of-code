use md5;

fn valid(hash: md5::Digest, zeros: &str) -> bool {
    let v = format!("{:x}", hash);
    &v[0..zeros.len()] == zeros
}

fn find_hash(key: &str, zeros1: &str, zeros2: &str) {
    let mut n: i64 = 0;
    let mut part1 = false;
    let mut part2 = false;
    loop {
        let digest = md5::compute(format!("{}{}", key, n));
        if !part1 && valid(digest, zeros1) {
            println!("part1: {}", n);
            part1 = true;
        }
        if !part2 && valid(digest, zeros2) {
            println!("part1: {}", n);
            part2 = true;
        }
        if part1 && part2 {
            break;
        }
        n += 1;
    }
}

fn main() {
    let input = "bgvyzdsv";

    find_hash(input, "00000", "000000");
}
