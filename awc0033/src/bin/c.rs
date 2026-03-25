use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        t: usize,
        c: [Usize1; k],
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1
        }
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut queue = VecDeque::new();
    let mut seen = vec![false; n];
    for &ci in &c {
        queue.push_back(ci);
        seen[ci] = true;
    }
    let mut cnts = vec![0; n];
    while let Some(v) = queue.pop_front() {
        let next = &graph[v];
        for &u in next {
            if seen[u] {
                continue;
            }

            cnts[u] += 1;
            if cnts[u] >= t {
                queue.push_back(u);
                seen[u] = true;
            }
        }
    }

    let ans = seen.iter().filter(|&&x| x).count();
    println!("{}", ans);
}
