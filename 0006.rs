use std::io::stdin;

fn main() {
    let mut buf = String::new();
    match stdin().read_line(&mut buf) {
        Ok(_) => {
            println!("{}", buf.trim().chars().rev().collect::<String>());
        }
        Err(_) => return,
    }
}
