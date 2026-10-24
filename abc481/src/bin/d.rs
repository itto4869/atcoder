use std::collections::VecDeque;

use cp_library::graph::dijkstra;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Usize1,
        t: Usize1,
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1
        }
        graph[a].push(b);
    }

    for i in 0..n {
        graph[i].sort_unstable();
    }

    let mut d_graph = vec![Vec::new(); n];
    for i in 0..n {
        if graph[i].is_empty() {
            continue;
        }

        let u0 = graph[i][0];
        d_graph[i].push((u0, 0));

        for &u in graph[i].iter().skip(1) {
            d_graph[i].push((u, 1));
        }
    }
    
    let dist = dijkstra(&d_graph, s);
    if dist[t] == usize::MAX {
        println!("-1");
    } else {
        println!("{}", dist[t]);
    }
}
