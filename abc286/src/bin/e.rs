use std::collections::VecDeque;

use proconio::{fastout, input, marker::{Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: [Chars; n],
        q: usize,
    }

    let mut dist = vec![vec![(usize::MAX, usize::MAX); n]; n];
    for i in 0..n {
        let mut queue = VecDeque::new();
        queue.push_back((i, 0, a[i]));
        dist[i][i] = (0, a[i]);
        while let Some((v, d, c)) = queue.pop_front() {
            for u in 0..n {
                if s[v][u] == 'N' {
                    continue;
                }

                let (nd, nc) = dist[i][u];
                if nd < (d + 1) {
                    continue;
                } else if nd == (d + 1) {
                    if nc >= (c + a[u]) {
                        continue;
                    } else {
                        dist[i][u] = (d + 1, c + a[u]);
                        queue.push_back((u, d + 1, c + a[u]));
                    }
                } else {
                    dist[i][u] = (d + 1, c + a[u]);
                    queue.push_back((u, d + 1, c + a[u]));
                }
            }
        }
    }

    for _ in 0..q {
        input! {
            u: Usize1,
            v: Usize1,
        }
        if dist[u][v] == (usize::MAX, usize::MAX) {
            println!("Impossible");
        } else {
            println!("{} {}", dist[u][v].0, dist[u][v].1);
        }
    }
}