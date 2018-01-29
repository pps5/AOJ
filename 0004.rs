use std::io::{stdin, BufRead, BufReader};

fn main() {
    let input = BufReader::new(stdin());
    for line in input.lines() {
        let v: Vec<f32> = line.unwrap()
            .split_whitespace()
            .filter_map(|x| x.parse::<f32>().ok())
            .collect();
        let x = (v[2] * v[4] - v[1] * v[5]) / (v[0] * v[4] - v[1] * v[3]);
        let y = (v[2] * v[3] - v[0] * v[5]) / (v[1] * v[3] - v[0] * v[4]);
        println!("{:.3} {:.3}", x, y);
    }
}
