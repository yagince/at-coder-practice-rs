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

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [chars; n],
        b: [chars; m],
    }

    // println!("{:?}", a);
    // println!("{:?}", b);

    for (i, a_row) in a.iter().enumerate() {
        if b.len() != 1 && (a.len() - i+1) < b.len() {
            break;
        }

        let ref b_first = b[0];
        let len = b_first.len();

        for (j, pair) in a_row.windows(len).enumerate() {
            if &pair.to_vec() == b_first {
                // println!("{} {}", i, j);
                // println!("a {:?}", pair);
                // println!("b {:?}", b_first);

                if ((i + 1)..(i + len))
                    .into_iter()
                    .enumerate()
                    .all(|(b_i, row_index)| {
                        let a_next = &a[row_index][j..j + len];
                        let ref b_next = b[b_i + 1];
                        // println!("{:?}", row_index);
                        // println!("a next {:?}", a_next);
                        // println!("b next {:?}", b_next);
                        // println!("{:?}", &a_next.to_vec() == b_next);
                        &a_next.to_vec() == b_next
                    })
                {
                    println!("Yes");
                    return
                }
            }
        }
    }

    println!("No");
}
