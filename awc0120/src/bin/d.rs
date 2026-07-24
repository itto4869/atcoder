use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        h: [usize; n],
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }
        let cost = h[u].abs_diff(h[v]);
        if cost > k {
            continue;
        } else {
            graph[u].push((v, cost));
            graph[v].push((u, cost));
        }
    }

    let mut dist = vec![usize::MAX; n];
    dist[0] = 0;
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), 0));
    while let Some((c, v)) = pq.pop() {
        let c = c.0;
        if c > dist[v] {
            continue;
        }

        let next = &graph[v];
        for &(nv, nc) in next {
            if dist[nv] > c + nc {
                dist[nv] = c + nc;
                pq.push((Reverse(c + nc), nv));
            }
        }
    }

    let ans = dist[n - 1];
    if ans == usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
