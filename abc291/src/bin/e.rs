use cp_graph::topological_sort;
use itertools::Itertools;
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
            x: Usize1,
            y: Usize1,
        }
        graph[x].push(y);
    }
    let Some(v) = topological_sort(&graph) else {
        println!("No");
        return;
    };

    if v.windows(2).any(|w| !graph[w[0]].contains(&w[1])) {
        println!("No");
        return;
    }

    let mut ans = vec![0; n];
    for (i, &v) in v.iter().enumerate() {
        ans[v] = i + 1;
    }

    println!("Yes\n{}", ans.iter().format(" "));
}