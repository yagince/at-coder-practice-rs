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
    let n = input::<usize>().to_string();
    let rev = n.chars().rev();
    if n.chars().collect::<Vec<_>>() == rev.collect::<Vec<_>>() {
        println!("Yes");
    } else {
        println!("No");
    }
}
