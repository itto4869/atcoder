use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        input! {
            u: Usize1,
            v: Usize1
        }
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut degree = vec![0; n];
    for i in 0..n {
        degree[i] = graph[i].len();
    }

    let mut seen = vec![false; n];
    let mut ans = Vec::new();
    for i in 0..n {
        if degree[i] >= 3 {
            seen[i] = true;
            ans.push(degree[i]);
            for &v in &graph[i] {
                seen[v] = true;
            }
        }
    }

    for i in 0..n {
        if degree[i] == 1 && !seen[i] {
            ans.push(2);
            seen[i] = true;
            let next = graph[i][0];
            seen[next] = true;
            for &u in &graph[next] {
                seen[u] = true;
            }
        }
    }

    let mut cnt = 0;
    for i in 0..n {
        if degree[i] == 2 && !seen[i] {
            cnt += 1;
        }
    }

    for _ in 0..(cnt / 3) {
        ans.push(2);
    }

    ans.sort_unstable();
    println!("{}", ans.iter().format(" "));
}