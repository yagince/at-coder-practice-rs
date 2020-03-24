use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    let _ = std::io::stdin().read_line(&mut input).unwrap();
    let input = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let n = input[0];
    let m = input[1];

    if m == 0 {
        println!("1");
        return;
    }

    let mut relations: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        relations.insert(i, 1 << i);
    }

    for _ in 0..m {
        let mut input = String::new();
        let _ = std::io::stdin().read_line(&mut input).unwrap();
        let input = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap() - 1)
            .collect::<Vec<_>>();
        if let Some(x) = relations.get_mut(&input[0]) {
            *x |= 1 << (input[1])
        }
        if let Some(x) = relations.get_mut(&input[1]) {
            *x |= 1 << (input[0])
        }
    }

    // すべての組み合わせを作る
    let v = (1..1 << n) // 人数分の各bitを順番に立てていくことで組み合わせになる
        .into_iter()
        .map(|group| {
            if (0..n)
                .into_iter()
                .filter(|i| group & (1 << *i) != 0) // この組み合わせに含まれる人を探す
                .fold(group, |acc, m| acc & relations.get(&m).unwrap()) // 組み合わせに含まれる人が全員知人が確認する
                == group
            {
                group.count_ones()
            } else {
                1
            }
        })
        .max();
    println!("{}", v.unwrap());
}
