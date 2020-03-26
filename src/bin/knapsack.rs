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

use std::cmp::max;

fn main() {
    input! {
        n: usize,
        w: usize,
        items: [(usize, usize); n],
    }

    let mut dp = vec![vec![0; w + 1]; n + 1];

    for i in 0..n {
        println!("------  {:?}", i);
        let (value, weight) = items[i];

        for total_weight in 0..(w + 1) {
            println!("  - - {:?}", total_weight);

            if total_weight >= weight {
                // このtotal_weightになる時に
                // 1. このitemの重さを足す前の価値にこのitemの価値を足した時の価値
                // 2. このitemの重さを足さずにこの重さになった時の価値
                // ↑このどちらか価値の高い方に更新する
                dp[i + 1][total_weight] = max(dp[i][total_weight - weight] + value, dp[i][total_weight]);
            } else {
                // total_weight を 超えたweightは足せないのでそのままにする
                dp[i + 1][total_weight] = dp[i][total_weight];
            }
        }
    }

    for t in dp.iter() {
        println!("{:?}", t);
    }

    println!("{:?}", dp[n].iter().max());
}
