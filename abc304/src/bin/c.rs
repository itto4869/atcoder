use std::collections::VecDeque;

use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: i64,
        xy: [(i64, i64); n],
    }
    let mut graph = vec![Vec::new(); n];
    for i in 0..n {
        for j in (i + 1)..n {
            let (xi, yi) = xy[i];
            let (xj, yj) = xy[j];
            let dist2 = (xi - xj) * (xi - xj) + (yi - yj) * (yi - yj);
            if dist2 <= d * d {
                graph[i].push(j);
                graph[j].push(i);
            }
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(0);
    let mut seen = vec![false; n];
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

    for f in seen {
        yes_no!(f);
    }
}
