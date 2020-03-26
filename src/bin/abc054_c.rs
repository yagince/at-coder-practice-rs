macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize1, usize1); m],
    }

    let edge_map = (0..n)
        .into_iter()
        .map(|i| {
            (
                i,
                edges
                    .iter()
                    .flat_map(|&(a, b)| match i {
                        _ if i == a => Some(b),
                        _ if i == b => Some(a),
                        _ => None,
                    })
                    .collect(),
            )
        })
        .collect::<HashMap<usize, Vec<usize>>>();
    println!("{}", walk(n, 0, &edge_map, vec![0]));
}

fn walk(n: usize, start: usize, edge_map: &HashMap<usize, Vec<usize>>, used: Vec<usize>) -> usize {
    let edges = edge_map.get(&start).unwrap();
    if edges.iter().all(|e| used.contains(e)) {
        if (0..n).all(|node| used.contains(&node)) {
            return 1
        } else {
            return 0
        }
    }

    edges.iter().filter(|e| !used.contains(e)).fold(0, |acc, e| {
        let mut used = used.clone();
        used.push(*e);
        acc + walk(n, *e, edge_map, used)
    })
}
