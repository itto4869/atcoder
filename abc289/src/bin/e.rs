use std::collections::VecDeque;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            c: [usize; n],
        }
        let mut graph = vec![Vec::new(); n];
        for _ in 0..m {
            input! {
                u: Usize1,
                v: Usize1,
            }
            graph[u].push(v);
            graph[v].push(u);
        }

        let mut ggraph = vec![vec![Vec::new(); n]; n];
        for i in 0..n {
            for j in 0..n {
                for &v in &graph[i] {
                    for &u in &graph[j] {
                        if c[v] != c[u] {
                            ggraph[i][j].push((v, u));
                        }
                    }
                }
            }
        }

        let mut queue = VecDeque::new();
        let mut dist = vec![vec![-1; n]; n];
        dist[0][n - 1] = 0;
        queue.push_back((0, n - 1));
        while let Some((v, u)) = queue.pop_front() {
            let next = &ggraph[v][u];
            for &(nv, nu) in next {
                if dist[nv][nu] != -1 {
                    continue;
                }

                dist[nv][nu] = dist[v][u] + 1;
                queue.push_back((nv, nu));
            }
        }

        let ans = dist[n - 1][0];
        println!("{}", ans);
    }
}
