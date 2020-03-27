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

// N A
// n1 n2 ...
fn main() {
    input! {
        n: usize,
        a: usize,
        numbers: [usize; n],
    }

    let mut dp = vec![vec![0; a+1]; n+1];
    dp[0][0] = 1; // 0個の整数の和は0なので1通り

    for i in 0..n {
        let num = numbers[i];

        for j in 0..(a+1) {
            // numを選択する場合 (i番目までいくつか選んでjにすることができるか)
            // i-1までにj-numにできる組み合わせがある場合はjにすることができる
            if j >= num {
                dp[i+1][j] = dp[i][j-num] + dp[i][j];
            } else {
                dp[i+1][j] = dp[i][j];
            }
        }
    }

    for x in dp.iter() {
        println!("{:?}", x);
    }

    println!("{}", dp[n][a]);
}
