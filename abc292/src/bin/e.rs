use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![Vec::new(); n];
    let mut edges = vec![vec![false; n]; n];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }
        graph[u].push(v);
        edges[u][v] = true;
    }

    let mut ans = 0;
    for i in 0..n {
        let mut queue = VecDeque::new();
        queue.push_back(i);
        let mut seen = vec![false; n];
        seen[i] = true;
        while let Some(v) = queue.pop_front() {
            let next = &graph[v];
            for &u in next {
                if seen[u] {
                    continue;
                }

                seen[u] = true;
                queue.push_back(u);
                ans += 1;
            }
        }
    }

    ans -= m;
    println!("{}", ans);
}
