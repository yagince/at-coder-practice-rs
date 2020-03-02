const PATTERNS: [&'static str; 4] = ["dreamer", "eraser", "dream", "erase"];

fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_owned();

    loop {
        if let Some(x) = PATTERNS.iter().find(|x| input.ends_with(**x)) {
            let len = input.len();
            input.drain((len - x.len())..len);
        } else {
            break;
        }
    }

    if input.len() == 0 {
        println!("YES");
    } else {
        println!("NO");
    }
}
