use std::collections::{HashMap, HashSet};
use std::io::prelude::*;

fn input<T>() -> T
where T: std::str::FromStr {
    let stdin = std::io::stdin();
    let token: String = stdin
        .lock()
        .bytes()
        .map(|c| c.unwrap() as char)
        .skip_while(|c| c.is_whitespace())
        .take_while(|c| !c.is_whitespace())
        .collect();
    token.parse().ok().unwrap()
}

fn main() {
    let n = input::<usize>();
    let m = input::<usize>();

    let mut routes: HashMap<usize, HashMap<usize, usize>> = (0..n)
        .into_iter()
        .map(|node| (node, HashMap::new()))
        .collect();
    let mut unused_routes: HashSet<(usize, usize)> = HashSet::new();

    for _ in 0..m {
        let a = input::<usize>() - 1;
        let b = input::<usize>() - 1;
        let cost = input::<usize>();
        if let Some(x) = routes.get_mut(&a) {
            x.insert(b, cost);
        }
        if let Some(x) = routes.get_mut(&b) {
            x.insert(a, cost);
        }
        unused_routes.insert((a, b));
    }
    // println!("{:?}", routes);

    let mut graph: Vec<Vec<Option<usize>>> = (0..n)
        .into_iter()
        .map(|node| {
            (0..n)
                .into_iter()
                .map(|i| routes.get(&node).unwrap().get(&i).map(|x| *x))
                .collect()
        })
        .collect();
    // println!("{:?}", graph);

    let next = warshall_floyd(&mut graph);
    // println!("{:?}", graph);
    // println!("{:?}", next);

    for s in 0..n {
        for e in 0..n {
            if s != e {
                if let Some((_, route)) = shortest_path(&graph, &next, s, e) {
                    for items in route.windows(2) {
                        unused_routes.remove(&(items[0], items[1]));
                        unused_routes.remove(&(items[1], items[0]));
                    }
                }
            }
        }
    }
    println!("{}", unused_routes.len());
}

fn warshall_floyd(dist: &mut Vec<Vec<Option<usize>>>) -> Vec<Vec<usize>> {
    let n = dist.len();
    let mut next = vec![];

    for _ in 0..n {
        next.push((0..n).collect::<Vec<usize>>());
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if let (Some(dik), Some(dkj)) = (dist[i][k], dist[k][j]) {
                    if dist[i][j].is_none() || dist[i][j].unwrap() > dik + dkj {
                        dist[i][j] = Some(dik + dkj);
                        next[i][j] = next[i][k];
                    }
                }
            }
        }
    }
    next
}

fn shortest_path(
    dist: &Vec<Vec<Option<usize>>>,
    next: &Vec<Vec<usize>>,
    start: usize,
    goal: usize,
) -> Option<(usize, Vec<usize>)> {
    if dist[start][goal].is_none() {
        return None;
    }

    let mut path = vec![start];
    let mut node = start;
    while node != goal {
        path.push(next[node][goal]);
        node = next[node][goal];
    }
    Some((dist[start][goal].unwrap(), path))
}
