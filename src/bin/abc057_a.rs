use std::io::prelude::*;

fn input<T>() -> T
where
    T: std::str::FromStr,
{
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
    let a = input::<u32>();
    let b = input::<u32>();
    println!("{:?}", (a + b) % 24);
}
