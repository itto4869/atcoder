use std::collections::HashMap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        input! {
            u: Usize1,
            v: Usize1,
        }
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut map = HashMap::new();
    let mut seen = vec![false; n];
    let mut pair = vec![false; n];
    seen[0] = true;
    map.insert(a[0], 1);
    dfs(0, &graph, &mut seen, &mut pair, &mut map, &a);

    for i in 0..n {
        if pair[i] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, seen: &mut Vec<bool>, pair: &mut Vec<bool>, map: &mut HashMap<u64, u64>, a: &Vec<u64>) {
    let next = &graph[v];
    for &u in next {
        if seen[u] {
            continue;
        }
        seen[u] = true;
        *map.entry(a[u]).or_insert(0) += 1;
        if *(map.get(&a[u]).unwrap()) >= 2 || pair[v] {
            pair[u] = true;
        }

        dfs(u, graph, seen, pair, map, a);

        let num = map.get(&a[u]).unwrap();
        map.insert(a[u], num - 1);
    }
}