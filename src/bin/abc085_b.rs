use std::collections::HashSet;
fn main() {
    let mut n = String::new();
    let _ = std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<u8>().unwrap();
    let mochis = (0..n)
        .map(|_| {
            let mut i = String::new();
            let _ = std::io::stdin().read_line(&mut i).unwrap();
            i.trim().parse::<u8>().unwrap()
        })
        .collect::<HashSet<_>>();
    println!("{}", mochis.len());
}
