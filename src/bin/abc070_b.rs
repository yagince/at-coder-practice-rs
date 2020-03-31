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
    let alice = (input::<u32>(), input::<u32>());
    let bob = (input::<u32>(), input::<u32>());

    let (first, second) = if alice.0 < bob.0 {
        (alice, bob)
    } else {
        (bob, alice)
    };

    if first.1 > second.0 {
        let e = std::cmp::min(second.1, first.1);
        println!("{}", e - second.0);
    } else {
        println!("0")
    }
}
