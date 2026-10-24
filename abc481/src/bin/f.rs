use std::{collections::{BinaryHeap, VecDeque}, fmt::Binary};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
            c: usize,
        }
        graph[a].push((b, c));
    }

    let mut queue = VecDeque::new();
    let mut dist = vec![-1; n];
    queue.push_back(0);
    dist[0] = 0;
    let mut pqv = vec![BinaryHeap::new(); n];
    while let Some(v) = queue.pop_front() {
        let next = &graph[v];
        for &(u, c) in next {
            if dist[u] > dist[v] + 1 {
                continue;
            }

            if dist[u] < dist[v] + 1 {
                dist[u] = dist[v] + 1;
                pqv[u] = BinaryHeap::new();
                pqv[u].push(c);
                queue.push_back(u);
            } else {
                pqv[u].push(c);
            }
        }
    }
}
