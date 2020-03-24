fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let input = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<_>>();

    let diff = (0 - input[0].0, 0 - input[0].1);
    let second = (input[1].0 + diff.0, input[1].1 + diff.1);
    let third = (input[2].0 + diff.0, input[2].1 + diff.1);
    println!(
        "{}",
        (second.0 * third.1 - second.1 * third.0).abs() as f32 / 2.0
    );
}
