use std::io::{stdin, BufRead, BufReader};

fn main() {
    let input = BufReader::new(stdin());
    for line in input.lines() {
        let mut v: Vec<u32> = line.unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();
        v.sort();
        println!("{} {}", lcm(v[1], v[0]), gcd(v[1], v[0]));
    }
}

fn lcm(a: u32, b: u32) -> u32 {
    let m = a % b;
    return match m {
        0 => b,
        _ => lcm(b, m),
    };
}

fn gcd(a: u32, b: u32) -> u64 {
    let l = lcm(a, b) as u64;
    let a = a as u64;
    let b = b as u64;
    return (a * b) / l;
}
