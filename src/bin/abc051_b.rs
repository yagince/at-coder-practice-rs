fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let input = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let k = input[0] + 1;
    let s = input[1];
    let mut count = 0;

    for x in 0..k {
        for y in 0..k {
            let t = x + y;
            if t > s {
                break;
            }
            if (s - t) < k {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
