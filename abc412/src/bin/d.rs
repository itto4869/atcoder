use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![vec![false; n]; n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1
        }
        graph[a][b] = true;
        graph[b][a] = true;
    }

    let mut ans = 1000;
    let x: Vec<usize> = (0..n).collect();
    for p in x.iter().permutations(n) {
        let mut cnt = 0;
        let mut comp = vec![vec![false; n]; n];
        for i in 0..n {
            let u = *p[i];
            let v = *p[(i + 1) % n];
            comp[u][v] = true;
            comp[v][u] = true;
        }

        for i in 0..n {
            for j in 0..n {
                if graph[i][j] != comp[i][j] {
                    cnt += 1;
                }
            }
        }
        ans = ans.min(cnt);
    }

    for p in x.iter().permutations(n) {
        for d in 3..=(n - 3) {
            let mut cnt = 0;
            let mut comp = vec![vec![false; n]; n];
            for i in 0..d {
                let u = *p[i];
                let v = *p[(i + 1) % d];
                comp[u][v] = true;
                comp[v][u] = true;
            }

            for i in d..n {
                let u = *p[i];
                let v = if i < n - 1 {
                    *p[i + 1]
                } else {
                    *p[d]
                };
                comp[u][v] = true;
                comp[v][u] = true;
            }

            for i in 0..n {
                for j in 0..n {
                    if graph[i][j] != comp[i][j] {
                        cnt += 1;
                    }
                }
            }

            ans = ans.min(cnt);
        }

    }

    println!("{}", ans / 2);
}
