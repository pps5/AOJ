use std::io::{stdin, BufRead, BufReader};

fn main() {
    let input = BufReader::new(stdin());
    for line in input.lines() {
        let mut values: Vec<u32> = line.unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();
        if values.len() == 1 {
            continue;
        }
        values.sort();
        let values: Vec<u32> = values.into_iter().map(|x| x.pow(2)).collect();
        if values[2] == (values[0] + values[1]) {
            println!("YES");
        } else {
            println!("NO");
        }

    }
}
