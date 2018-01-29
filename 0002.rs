use std::io::{stdin, BufRead, BufReader};

fn main() {
    let input = BufReader::new(stdin());
    for line in input.lines() {
        let values: Vec<u32> = line.unwrap()
            .split(' ')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();
        let sum = (values[0] + values[1]).to_string();
        println!("{}", sum.len());
    }
}
