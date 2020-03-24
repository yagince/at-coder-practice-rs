fn main() {
    let mut n = String::new();
    let _ = std::io::stdin().read_line(&mut n).unwrap();
    let n = n.trim().parse::<i32>().unwrap();

    let routes = (0..n)
        .into_iter()
        .map(|_| {
            let mut input = String::new();
            let _ = std::io::stdin().read_line(&mut input).unwrap();
            let input = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            (input[0], input[1], input[2])
        })
        .collect::<Vec<(i32, i32, i32)>>();

    let mut current = (0, 0, 0);
    for route in routes {
        if walk(&current, &route) {
            current = route;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn walk(start: &(i32, i32, i32), end: &(i32, i32, i32)) -> bool {
    let diff = (end.0 - start.0, end.1 - start.1, end.2 - start.2);
    let total = diff.1 + diff.2;
    let t = diff.0;
    total <= t && even(t) == even(total)
}

fn even(i: i32) -> bool {
    i % 2 == 0
}
