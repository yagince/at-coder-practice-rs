fn main() {
    let boin = ['a', 'i', 'u', 'e', 'o'];
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    println!(
        "{}",
        input
            .trim()
            .chars()
            .filter(|x| !boin.contains(x))
            .collect::<String>()
    );
}
