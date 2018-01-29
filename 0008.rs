use std::io::{stdin, BufRead, BufReader};
fn main() {
    let input = BufReader::new(stdin());
    for line in input.lines() {
        let n: u32 = line.unwrap().trim().parse::<u32>().unwrap();
        println!("{}", combination(n));
    }
}

fn combination(n: u32) -> u32 {
    let mut c = 0;
    for i in 0..10 {
        for j in 0..10 {
            for k in 0..10 {
                for l in 0..10 {
                    if i + j + k + l == n {
                        c += 1;
                    }
                }
            }
        }
    }
    return c;
}
