use cp_library::graph::topological_sort;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }
        graph[u].push(v);
    }

    let sort_graph = topological_sort(&graph);
    if sort_graph.is_none() {
        println!("-1");
        return;
    }

    let mut dist = a.clone();
    let sort_graph = sort_graph.unwrap();
    for u in sort_graph {
        for &v in &graph[u] {
            dist[v] = dist[v].max(dist[u] + a[v]);
        }
    }

    let ans = dist.into_iter().max().unwrap();
    println!("{}", ans);
}
