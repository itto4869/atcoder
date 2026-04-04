use std::collections::HashMap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        p: [Usize1; n - 1],
    }
    let mut graph = vec![Vec::new(); n];
    for i in 0..(n - 1) {
        let pi = p[i];
        graph[pi].push(i + 1);
    }
    let mut cnt = vec![0; n];

    for _ in 0..m {
        input! {
            x: Usize1,
            y: u64,
        }
        cnt[x] = cnt[x].max(y + 1);
    }

    let mut seen = vec![false; n];
    for i in 0..n {
        if seen[i] {
            continue;
        }

        seen[i] = true;
        dfs(i, &graph, &mut cnt, &mut seen);
    }

    let ans = cnt.iter().filter(|&&c| c > 0).count();
    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, cnt: &mut Vec<u64>, seen: &mut Vec<bool>) {
    let next = &graph[v];
    for &u in next {
        if cnt[u] >= cnt[v].saturating_sub(1) {
            continue;
        }

        seen[u] = true;
        cnt[u] = cnt[v] - 1;
        dfs(u, graph, cnt, seen);
    }
}