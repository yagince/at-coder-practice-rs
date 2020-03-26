use std::io::prelude::*;

fn input<T>() -> T
where T: std::str::FromStr {
    let stdin = std::io::stdin();
    let token: String = stdin
        .lock()
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().unwrap()
}

fn main() {
    let a = match input::<u32>() {
        1 => 14,
        x => x,
    };
    let b = match input::<u32>() {
        1 => 14,
        x => x,
    };
    if a == b {
        println!("Draw");
    } else if a > b {
        println!("Alice");
    } else {
        println!("Bob");
    }
}
