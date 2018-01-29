use std::io::{stdin, BufRead, BufReader};

fn main() {
    let input = BufReader::new(stdin());
    for line in input.lines() {
        let d = line.unwrap().parse::<u32>().unwrap();
        let mut s = 0;
        let mut x = d;
        while x < 600 {
            s += x * x * d;
            x += d;
        }
        println!("{}", s);
    }
}
