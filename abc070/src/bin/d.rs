use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        abc: [(Usize1, Usize1, i64); n - 1],
        q: usize,
        k: Usize1,
        xy: [(Usize1, Usize1); q],
    }

    let mut graph = vec![Vec::new(); n];
    for (a, b, c) in abc {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut dists: Vec<i64> = vec![-1; n];
    dists[k] = 0;
    let mut q = VecDeque::new();
    q.push_back(k);
    while !q.is_empty() {
        let now = q.pop_front().unwrap();
        for (next, cost) in &graph[now] {
            if dists[*next] == -1 {
                dists[*next] = dists[now] + cost;
                q.push_back(*next);
            }
        }
    }

    for (x, y) in xy {
        let ans = dists[x] + dists[y];
        println!("{}", ans);
    }
}