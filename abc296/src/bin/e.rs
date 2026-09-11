use cp_library::graph::FunctionalGraph;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    }
    let graph = FunctionalGraph::new(a);
    let cycles = graph.cycles();

    let mut ans = 0;
    for cycle in cycles {
        ans += cycle.len();
    }

    println!("{}", ans);
}
