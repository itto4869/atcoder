use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n1: usize,
        n2: usize,
        m: usize,
    }
    let mut graph = vec![Vec::new(); n1 + n2];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut queue = VecDeque::new();
    let mut dist = vec![-1; n1 + n2];
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

    queue.push_back(n1 + n2 - 1);
    dist[n1 + n2 - 1] = 1;
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

    let mut d1 = 0;
    for i in 0..n1 {
        d1 = d1.max(dist[i]);
    }

    let mut d2 = 0;
    for i in n1..(n1 + n2) {
        d2 = d2.max(dist[i]);
    }

    let ans = d1 + d2;
    println!("{}", ans);
}
