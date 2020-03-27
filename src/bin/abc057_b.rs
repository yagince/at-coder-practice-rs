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

use std::collections::{HashMap, BinaryHeap};

fn main() {
    input! {
        n: usize,
        m: usize,
        students: [(i32, i32); n],
        check_points: [(i32, i32); m],
    }

    for student in students {
        let mut diffs = BinaryHeap::new();
        let mut diff_map: HashMap<i32, BinaryHeap<i32>> = HashMap::new();

        for (i, point) in check_points.iter().enumerate() {
            let diff = diff(&student, point);
            let v = diff_map.entry(-diff).or_insert(BinaryHeap::new());
            v.push(i as i32 * -1);
            diffs.push(-diff);
        }

        let min = diffs.pop().unwrap();
        let mut indexes = diff_map.get_mut(&min).unwrap();
        println!("{}", indexes.pop().unwrap() * -1 + 1);
    }
}

fn diff(&(s_x, s_y): &(i32, i32), &(c_x, c_y): &(i32, i32)) -> i32 {
    (s_x - c_x).abs() + (s_y - c_y).abs()
}
