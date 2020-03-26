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

const ITEM_MAX: usize = 401;
const INF: usize = 10_000_000;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        m: (usize, usize),
        items: [((usize, usize), usize); n],
    }

    let mut dp = vec![vec![vec![INF; ITEM_MAX]; ITEM_MAX]; n + 1];

    dp[0][0][0] = 0;

    for i in 0..n {
        for j in 0..ITEM_MAX {
            for k in 0..ITEM_MAX {
                if dp[i][j][k] == INF {
                    continue;
                }
                // 加えない場合
                // なにも変わらないので、次の値と比較して小さい方で更新
                dp[i + 1][j][k] = min(dp[i + 1][j][k], dp[i][j][k]);

                // 加える場合
                let ((a, b), c) = items[i];
                let next_a = j + a;
                let next_b = k + b;
                // 他のパターンですでに更新されている場合もありうるので、今の値にCを足したものと、次のテーブルの対象の値とを比較して小さい方で更新
                dp[i + 1][next_a][next_b] = min(dp[i + 1][next_a][next_b], dp[i][j][k] + c);
            }
        }
    }

    let mut min_c = None;

    // 一番小さいCの合計を探す
    for j in 1..ITEM_MAX {
        for k in 1..ITEM_MAX {
            // 比率が同じ場合のみ
            // a:b = Ma:Mb => a*Mn == b*Ma
            let c = dp[n][j][k];
            if j * m.1 == k * m.0  && c != INF {
                min_c = min_c.map(|x| min(x, c)).or(Some(c));
            }
        }
    }
    match min_c {
        Some(x) => println!("{:?}", x),
        _ => println!("-1"),
    }
}
