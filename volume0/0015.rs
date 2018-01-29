use std::io::stdin;

fn main() {
    let mut buf = String::new();
    let input = stdin();
    input.read_line(&mut buf).unwrap();
    for _ in 0..buf.trim().parse::<u32>().unwrap() {
        let mut v1 = String::new();
        let mut v2 = String::new();
        input.read_line(&mut v1).unwrap();
        input.read_line(&mut v2).unwrap();
        if v1.trim().len() > 80 || v2.trim().len() > 80 {
            println!("overflow");
        } else {
            let mut v1 = v1.trim().
            let v1: u64 = v1.trim().parse().unwrap();
            let v2: u64 = v2.trim().parse().unwrap();
            let sum = (v1 + v2).to_string();
            if sum.len() > 80 {
                println!("overflow");
            } else {
                println!("{}", sum);
            }
        }
    }
}
