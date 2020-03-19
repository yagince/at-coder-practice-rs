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
        let diff = (route.1 - current.1, route.2 - current.2);
        if walk(route.0 - current.0, diff) {
            current = route;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn walk(t: i32, point: (i32, i32)) -> bool {
    let total = point.0 + point.1;
    total <= t && even(t) == even(total)
}

fn even(i: i32) -> bool {
    i % 2 == 0
}
