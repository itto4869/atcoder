use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        m: usize,
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut seen = vec![vec![false; n]; n];
    let mut cnts = vec![t; n];
    for i in 0..n {
        if seen[i][i] {
            continue;
        }
        seen[i][i] = true;
        dfs(i, &graph, &mut seen, &mut cnts);
    }

    let mut ans = 1u64;
    for &c in &cnts {
        ans *= c as u64;
    }

    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<Vec<bool>>, cnts: &mut Vec<usize>) {
    let next = &graph[v];
    for &u in next {
        if seen[v][u] || seen[u][v] {
            continue;
        }

        seen[v][u] = true;
        seen[u][v] = true;
        cnts[u] -= 1;
        dfs(u, graph, seen, cnts);
    }
}