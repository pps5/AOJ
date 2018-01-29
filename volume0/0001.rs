use std::io::{stdin, BufReader, BufRead};

fn main() {
    let input = BufReader::new(stdin());
    let mut heights: Vec<u32> = Vec::new();

    for line in input.lines() {
        match line.unwrap().trim().parse() {
            Ok(v) => heights.push(v),
            Err(_) => break,
        }
    }
    let heights = heights.as_mut_slice();
    heights.sort();
    heights.reverse();
    for h in heights[0..3].iter() {
        println!("{}", h);
    }
}
