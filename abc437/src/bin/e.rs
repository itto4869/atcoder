use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut graph = vec![Vec::new(); n + 1];
    for i in 1..=n {
        input! {
            x: usize,
            y: u64,
        }
        graph[x].push((i, y));
    }

    let mut ans = Vec::with_capacity(n);
    dfs(vec![0], &graph, &mut ans);

    println!("{}", ans.iter().format(" "));
}

fn dfs(nodes: Vec<usize>, graph: &Vec<Vec<(usize, u64)>>, ans: &mut Vec<usize>) {
    let mut trans = Vec::new();
    for &u in &nodes {
        for &(v, y) in &graph[u] {
            trans.push((y, v));
        }
    }

    trans.sort();

    for (_, group) in &trans.iter().group_by(|t| t.0) {
        let mut next_nodes = Vec::new();
        for &(_, v) in group {
            ans.push(v);
            next_nodes.push(v);
        }

        dfs(next_nodes, graph, ans);
    }
}