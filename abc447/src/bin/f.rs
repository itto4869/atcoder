use std::collections::VecDeque;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    for _ in 0..q {
        input! {
            n: usize,
        }
        let mut graph = vec![Vec::new(); n];
        for _ in 0..(n - 1) {
            input! {
                a: Usize1,
                b: Usize1,
            }
            graph[a].push(b);
            graph[b].push(a);
        }
        let mut dist = vec![-1; n];
        let mut queue = VecDeque::new();
        for i in 0..n {
            if graph[i].len() < 3 {
                continue;
            }
            if dist[i] != -1 {
                continue;
            }
            dist[i] = 0;
            queue.push_back(i);
            while let Some(v) = queue.pop_front() {
                let next = &graph[v];
                for &u in next {
                    if dist[u] != -1 {
                        continue;
                    }

                    if graph[u].len() < 3 {
                        continue;
                    }

                    if graph[u].len() == 2 {
                        dist[u] = dist[v] + 1;
                    } else {
                        dist[u] = dist[v] + 1;
                        queue.push_back(u);
                    }
                }
            }

            let &ans = dist.iter().max().unwrap();
            if ans == -1 {
                println!("0")
            } else {
                println!("{}", ans);
            }
        }

    }
}
