#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let input = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let s = Point {
        x: input[0],
        y: input[1],
    };
    let t = Point {
        x: input[2],
        y: input[3],
    };

    let mut route = String::new();

    for _ in s.x..t.x {
        route.push('R');
    }
    for _ in s.y..t.y {
        route.push('U');
    }
    for _ in s.x..t.x {
        route.push('L');
    }
    for _ in s.y..t.y {
        route.push('D');
    }

    // sまで戻ったので、1マスしたにずらして、1マス分外側の最短経路を進む
    route.push('D');

    // 1マスそと
    for _ in s.x..t.x + 1 {
        route.push('R');
    }
    for _ in s.y - 1..t.y {
        route.push('U');
    }
    // 1個左に行ってtに到着
    route.push('L');

    // そのまま左に行くとすでに通過した経路なので、1個上にずらす
    route.push('U');

    for _ in s.x..t.x + 1 {
        route.push('L');
    }
    for _ in s.y - 1..t.y {
        route.push('D');
    }

    route.push('R');
    println!("{}", route);
}
