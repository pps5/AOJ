use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut stack = vec![];
    let input = BufReader::new(stdin());
    for line in input.lines() {
        let n = line.unwrap().trim().parse::<u32>().unwrap();
        if n == 0 {
            println!("{}", stack.last().unwrap());
            let last_idx = stack.len() - 1;
            stack.remove(last_idx);
        } else {
            stack.push(n);
        }
    }
}
