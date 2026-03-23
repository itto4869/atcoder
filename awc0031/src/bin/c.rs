use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: i64,
        s: Usize1,
        t: Usize1,
        xy: [(i64, i64); n],
    }
    let mut graph = vec![Vec::new(); n];
    for i in 0..n {
        for j in (i + 1)..n {
            let (xi, yi) = xy[i];
            let (xj, yj) = xy[j];
            let diff = (xi - xj) * (xi - xj) + (yi - yj) * (yi - yj);
            if diff > d * d {
                continue;
            }

            graph[i].push(j);
            graph[j].push(i);
        }
    }

    let mut diff = vec![-1; n];
    let mut queue = VecDeque::new();
    queue.push_back(s);
    diff[s] = 0;
    while let Some(v) = queue.pop_front() {
        let next = &graph[v];
        for &u in next {
            if diff[u] != -1 {
                continue;
            }

            diff[u] = diff[v] + 1;
            queue.push_back(u);
        }
    }

    println!("{}", diff[t]);
}
