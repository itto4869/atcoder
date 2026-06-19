use std::process::exit;

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
            u: Usize1,
            v: Usize1,
        }
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut visited = vec![false; n];
    visited[0] = true;
    let mut k = 1;
    dfs(0, &graph, &mut k, &mut visited);
    println!("{}", k);
}

fn dfs(u: usize, graph: &Vec<Vec<usize>>, k: &mut usize, visited: &mut Vec<bool>) {
    if *k > 100_0000 {
        println!("1000000");
        exit(0);
    }

    let next = &graph[u];
    for &v in next {
        if visited[v] {
            continue;
        }

        visited[v] = true;
        *k = *k + 1;
        dfs(v, graph, k, visited);
        visited[v] = false;
    }
}