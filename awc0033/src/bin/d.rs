use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: u64,
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
            c: u64,
        }
        graph[u].push((v, c));
        graph[v].push((u, c));
    }

    let mut dist = vec![u64::MAX; n];
    let mut heap = BinaryHeap::new();
    heap.push((Reverse(0), 0));
    while let Some((d, v)) = heap.pop() {
        let d = d.0;
        if d > dist[v] {
            continue;
        }

        let next = &graph[v];
        for &(u, c) in next {
            if d + c >= dist[u] {
                continue;
            }
            dist[u] = d + c;
            heap.push((Reverse(d + c), u));
        }
    }

    if dist[n - 1] <= k {
        println!("{}", dist[n - 1]);
    } else {
        println!("-1");
    }
}
