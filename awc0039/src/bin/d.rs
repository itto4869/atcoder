use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut map: HashMap<(usize, usize), u64> = HashMap::new();

    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
            c: u64,
        }
        map.entry((u, v))
            .and_modify(|x| *x = (*x).min(c))
            .or_insert(c);
    }

    let mut graph = vec![Vec::new(); n];
    for ((u, v), c) in map {
        graph[u].push((v, c));
    }

    let mut dist = vec![u64::MAX; n];
    let mut heap = BinaryHeap::new();

    dist[0] = 0;
    heap.push((Reverse(0_u64), 0usize));

    while let Some((Reverse(d), v)) = heap.pop() {
        if d > dist[v] {
            continue;
        }

        if v == n - 1 {
            break;
        }

        for &(to, cost) in &graph[v] {
            let nd = d + cost;
            if nd < dist[to] {
                dist[to] = nd;
                heap.push((Reverse(nd), to));
            }
        }
    }

    println!("{}", dist[n - 1]);
}