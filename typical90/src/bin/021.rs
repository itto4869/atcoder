use ac_library::SccGraph;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = SccGraph::new(n);
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        graph.add_edge(a, b);
    }

    let groups = graph.scc();
    let mut ans = 0;
    for group in groups {
        let cnt = group.len();
        ans += (cnt * (cnt.saturating_sub(1))) / 2;
    }

    println!("{}", ans);
}