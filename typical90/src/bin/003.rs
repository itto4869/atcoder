use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        input! {
            a: Usize1,
            b: Usize1
        }
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut queue = VecDeque::new();
    let mut dist = vec![-1; n];
    queue.push_back(0);
    dist[0] = 0;
    while let Some(v) = queue.pop_front() {
        let next = &graph[v];
        for &u in next {
            if dist[u] != -1 {
                continue;
            }

            dist[u] = dist[v] + 1;
            queue.push_back(u);
        }
    }

    let s = dist.iter().position_max().unwrap();
    dist = vec![-1; n];
    queue.push_back(s);
    dist[s] = 0;
    while let Some(v) = queue.pop_front() {
        let next = &graph[v];
        for &u in next {
            if dist[u] != -1 {
                continue;
            }

            dist[u] = dist[v] + 1;
            queue.push_back(u);
        }
    }

    let ans = dist.iter().max().unwrap() + 1;
    println!("{}", ans);
}
