use cp_library::graph::dijkstra;
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
            a: Usize1,
            b: Usize1,
            c: usize,
        }
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let dist1 = dijkstra(&graph, 0);
    let dist2 = dijkstra(&graph, n - 1);

    for k in 0..n {
        let ans = dist1[k] + dist2[k];
        println!("{}", ans);
    }
}
