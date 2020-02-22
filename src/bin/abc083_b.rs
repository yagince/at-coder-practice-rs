fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let inputs = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let n = inputs[0];
    let a = inputs[1];
    let b = inputs[2];

    let r = (1..n + 1)
        .flat_map(|x| {
            let sum = x.to_string()
                .chars()
                .fold(0, |acc, i| acc + i.to_digit(10).unwrap());
            if a <= sum && sum <= b {
                Some(x)
            } else {
                None
            }
        })
        .fold(0, |acc, i| acc + i);

    println!("{:?}", r);
}
