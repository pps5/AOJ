use std::io::{stdin, BufRead, BufReader};

fn main() {
    let mut primes: Vec<bool> = Vec::with_capacity(1000000);

    for _ in 0..1000000 {
        primes.push(true);
    }
    primes[0] = false;
    primes[1] = false;
    for i in 2..1000000 {
        if primes[i] {
            let mut j = i + i;
            while j < primes.len() {
                primes[j] = false;
                j += i;
            }
        }
    }
    let input = BufReader::new(stdin());
    for line in input.lines() {
        let n: usize = line.unwrap().trim().parse::<usize>().unwrap() + 1;
        let mut c = 0;
        for i in 2..n {
            if primes[i] {
                c += 1;
            }
        }
        println!("{}", c);
    }
}
