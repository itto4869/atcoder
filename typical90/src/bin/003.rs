use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![Vec::new(); n];
    for (a, b) in ab {
        graph[a].push(b);
        graph[b].push(a);
    }
    
    let mut dist: Vec<Option<u64>> = vec![None; n];
    let mut q = VecDeque::new();
    q.push_back(0);
    dist[0] = Some(0);
    while let Some(u) = q.pop_front() {
        for &next in &graph[u] {
            if dist[next].is_none() {
                dist[next] = Some(dist[u].unwrap() + 1);
                q.push_back(next);
            }
        }
    }
    let mut max_dist = 0;
    let mut new_point = 0;
    for (p, d) in dist.iter().enumerate() {
        if max_dist < d.unwrap() {
            max_dist = d.unwrap();
            new_point = p;
        }
    }

    let mut dist: Vec<Option<u64>> = vec![None; n];
    let mut q = VecDeque::new();
    q.push_back(new_point);
    dist[new_point] = Some(0);
    while let Some(u) = q.pop_front() {
        for &next in &graph[u] {
            if dist[next].is_none() {
                dist[next] = Some(dist[u].unwrap() + 1);
                q.push_back(next);
            }
        }
    }
    let mut max_dist = 0;
    for d in dist {
        max_dist = max_dist.max(d.unwrap());
    }
    println!("{}", max_dist + 1);
}
