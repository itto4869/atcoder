use std::collections::VecDeque;

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
            w: u64,
        }
        graph[u].push((v, w));
        graph[v].push((u, w));
    }
    
    let mut queue = VecDeque::new();
    let mut diff = vec![-1; n];
    queue.push_back(0);
    diff[0] = 0;
    while let Some(v) = queue.pop_front() {
        let next = &graph[v];
        for &(u, wi) in next {
            if diff[u] != -1 || wi < k {
                continue;
            }

            diff[u] = diff[v] + 1;
            queue.push_back(u);
        }
    }

    println!("{}", diff[n - 1]);
}
