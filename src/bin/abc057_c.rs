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

use std::collections::HashSet;

fn main() {
    let n = input::<usize>();

    let mut combi: HashSet<usize> = HashSet::new();

    for i in 1..((n as f64).sqrt() as usize + 1) {
        if n % i == 0 {
            let i_len = digits(i);
            let j_len = digits(n / i);
            let count = std::cmp::max(i_len, j_len);
            combi.insert(count);
        }
    }
    println!("{:?}", combi.iter().min().unwrap());
}

fn digits(i: usize) -> usize {
    format!("{}", i).len()
}
