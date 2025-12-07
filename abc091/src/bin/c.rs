use ac_library::MfGraph;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(u64, u64); n],
        cd: [(u64, u64); n],
    }
    let mut mfgraph = MfGraph::new(2 * n + 2);
    for i in 0..n {
        mfgraph.add_edge(0, i + 1, 1);
        mfgraph.add_edge(n + i + 1, 2 * n + 1, 1);
    }
    for i in 0..n {
        for j in 0..n {
            let (a, b) = ab[i];
            let (c, d) = cd[j];
            if a < c && b < d {
                mfgraph.add_edge(i + 1, n + j + 1, 1);
            }
        }
    }

    let max_flow = mfgraph.flow(0, 2 * n + 1);
    println!("{}", max_flow);
}
