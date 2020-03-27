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

const INF: usize = 1 << 29;

/// N個の整数から何個か選択し総和がAになる整数の個数の最小値
///
/// N A
/// n1 n2 ...
fn main() {
    input! {
        n: usize,
        a: usize,
        numbers: [usize; n],
    }

    // 最小値を取りたいので、初期値を大きい値にする
    let mut dp = vec![vec![INF; a+1]; n+1];
    dp[0][0] = 0;

    println!("N: {:?}", n);
    println!("A: {:?}", a);

    for i in 0..n {
        let num = numbers[i];
        println!("--------  {} : {}", i, num);

        println!("{:?}", dp[i]);
        for j in 0..(a+1) {
            // numを選択する場合 (i番目までいくつか選んでjにすることができる個数)
            // i-1までにj-numにできる組み合わせの個数に１通り増えるので+1する のと比較して小さくなるようであれば更新する
            dp[i+1][j] = std::cmp::min(dp[i+1][j], dp[i][j]);
            if j >= num {
                dp[i+1][j] = std::cmp::min(dp[i+1][j], dp[i][j-num] + 1);
            }
        }
        println!("{:?}", dp[i+1]);
    }

    if dp[n][a] < INF {
        println!("{}", dp[n][a]);
    } else {
        println!("-1");
    }
}
