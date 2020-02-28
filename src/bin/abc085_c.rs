fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let inputs = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let count = inputs[0];
    let total = inputs[1];

    if let Some(x) = calc_all(count, total) {
        println!("{} {} {}", x.0, x.1, x.2);
    } else {
        println!("-1 -1 -1");
    }
}

fn calc_all(count: i32, total: i32) -> Option<(i32, i32, i32)> {
    let mut x = count;
    while x >= 0 {
        let r = calc_x(count, total, x);
        if r.is_some() {
            return r;
        }
        x -= 1;
    }
    None
}

fn calc_x(count: i32, total: i32, x: i32) -> Option<(i32, i32, i32)> {
    let mut y = count - x;
    while y >= 0 {
        let r = calc_y(count, total, x, y);
        if r.is_some() {
            return r;
        }
        y -= 1;
    }
    None
}

fn calc_y(count: i32, total: i32, x: i32, y: i32) -> Option<(i32, i32, i32)> {
    calc_z(total, x, y, count - (x + y))
}

fn calc_z(total: i32, x: i32, y: i32, z: i32) -> Option<(i32, i32, i32)> {
    if 10000 * x + 5000 * y + 1000 * z == total {
        Some((x, y, z))
    } else {
        None
    }
}
