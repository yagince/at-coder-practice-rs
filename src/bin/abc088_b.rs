fn main() {
    let mut n = String::new();
    let _ = std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<usize>().unwrap();

    let mut cards = String::new();
    let _ = std::io::stdin().read_line(&mut cards).unwrap();
    let mut cards = cards
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .take(n)
        .collect::<Vec<_>>();
    cards.sort();

    let mut alice = vec![];
    let mut bob = vec![];

    let mut i = 0;
    while let Some(card) = cards.pop() {
        if i % 2 == 0 {
            alice.push(card);
        } else {
            bob.push(card);
        }
        i += 1;
    }
    println!("{}", alice.into_iter().sum::<usize>() - bob.into_iter().sum::<usize>());
}
