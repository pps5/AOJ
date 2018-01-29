use std::io::stdin;

fn main() {
    let mut debt = 100000;
    let mut buf = String::new();
    match stdin().read_line(&mut buf) {
        Ok(_) => {
            let week: u32 = buf.trim().parse::<u32>().unwrap();
            for _ in 1..week + 1 {
                let interest = debt * 5 / 100;
                debt += (interest / 1000) * 1000;
                if interest % 1000 > 0 {
                    debt += 1000;
                }
            }
            println!("{}", debt);
        }
        Err(_) => return,
    }
}
