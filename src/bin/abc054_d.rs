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
        m: (u32, u32),
        items: [((u32, u32), u32); n],
    }

    let mut costs = items.iter().map(|&((_, _), c)| c).collect::<Vec<_>>();
    costs.sort();

    let mut min = None;

    for i in 1..n + 1 {
        if i >= costs.len() {
            break;
        }
        println!("--- {:?}", i);
        if let Some(x) = Combinations::new((0..n).collect(), i)
            .flat_map(|group| {
                println!("{:?}", group);
                let ((a, b), c) = group.iter().fold(((0, 0), 0), |acc, i| {
                    let ((i_a, i_b), i_c) = items[*i];
                    let ((a_a, a_b), a_c) = acc;
                    ((i_a + a_a, i_b + a_b), i_c + a_c)
                });
                let g = std::cmp::max(gcd(a, b), 1);
                if (a / g, b / g) == m {
                    Some(c)
                } else {
                    None
                }
            })
            .min()
        {
            println!("found {:?}", x);
            match min {
                Some(before) => {
                    println!("before {:?}", before);
                    println!("lower {:?}", costs.iter().take(i+1).sum::<u32>());
                    if x < before {
                       min = Some(x)
                    }
                    if min.unwrap() <= costs.iter().take(i+1).sum() {
                        println!("break");
                        break;
                    }
                }
                None => min = Some(x),
            }
        }
    }

    match min {
        Some(x) => println!("{:?}", x),
        None => println!("-1"),
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    match a {
        0 => b,
        _ => gcd(b % a, a),
    }
}

pub struct Combinations<T>
where
    T: Ord + Clone,
{
    original: Vec<T>,
    possition: Vec<usize>,
    len: usize,
    started: bool,
}

impl<T> Combinations<T>
where
    T: Ord + Clone,
{
    pub fn new(mut original: Vec<T>, len: usize) -> Self {
        if original.len() > len && len >= 1 {
            original.sort();
            Combinations {
                original: original,
                possition: (0..len).collect(),
                len: len,
                started: false,
            }
        } else {
            panic!("the length has to be smaller then the datasets len");
        }
    }

    #[inline]
    fn insert(&self, col: &mut Vec<T>) {
        col.clear();
        for (p, n) in self.possition.iter().enumerate() {
            col.insert(p, self.original[*n].clone())
        }
    }

    pub fn next_combination(&mut self, mut comb: &mut Vec<T>) -> bool {
        if !self.started {
            // first pass throught
            self.started = true;
            self.insert(&mut comb);
            true
        } else {
            let org_len = self.original.len();
            // check if we cant bump the back number
            if self.original[self.possition[self.len - 1]] == self.original[org_len - 1] {
                // locate the number closest behind that needs to be bumped
                for i in 2..(self.len + 1) {
                    if self.original[self.possition[self.len - i]] < self.original[org_len - i] {
                        //find the value of the
                        let lastpos = self.possition[self.len - i];
                        let val = &self.original[lastpos];
                        for j in lastpos + 1..org_len {
                            if *val < self.original[j] {
                                for k in 0..i {
                                    self.possition[self.len - i + k] = j + k;
                                }
                                self.insert(&mut comb);
                                return true;
                            }
                        }
                    }
                }
                false
            } else {
                let mut i = self.possition[self.len - 1];
                let current = &self.original[i];
                let mut next = current;
                while current == next {
                    i += 1;
                    next = &self.original[i];
                }
                self.possition[self.len - 1] = i;
                self.insert(&mut comb);
                true
            }
        }
    }
}

impl<T> Iterator for Combinations<T>
where
    T: Ord + Clone,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut vals = Vec::with_capacity(self.len);
        if self.next_combination(&mut vals) {
            Some(vals)
        } else {
            None
        }
    }
}
