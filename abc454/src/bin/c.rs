use std::collections::VecDeque;

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
        }
        graph[a].push(b);
    }

    let mut queue = VecDeque::new();
    let mut seen = vec![false; n];
    queue.push_back(0);
    seen[0] = true;
    while let Some(v) = queue.pop_front() {
        let next = &graph[v];
        for &u in next {
            if seen[u] {
                continue;
            }

            seen[u] = true;
            queue.push_back(u);
        }
    }

    let ans = seen.iter().filter(|&&x| x).count();
    println!("{}", ans);
}
