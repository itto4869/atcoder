use std::collections::{HashMap, VecDeque};

use ac_library::Dsu;
use cp_library::{data_structure::binary_trie::BinaryTrie, graph::rooted_tree};
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }
    let mut graph = vec![Vec::new(); n];
    for &(a, b) in &ab {
        graph[a].push(b);
        graph[b].push(a);
    }

    let (parent, order) = rooted_tree(&graph, 0);

    let mut subtree = vec![1usize; n];
    for &v in order.iter().rev() {
        if v != 0 {
            subtree[parent[v]] += subtree[v];
        }
    }



    let mut ans = vec![0i64; n];
    ans[0] = (n as i64) * (n as i64 - 1) / 2;

    for (i, &(a, b)) in ab.iter().enumerate() {
        let child = if parent[a] == b { a } else { b };
        let size = subtree[child];
        let w = (i + 1) as i64;

        if size * 2 < n {
            ans[child] -= w;
        } else {
            ans[0] -= w;
            ans[child] += w;
        }
    }

    for &v in order.iter().skip(1) {
        ans[v] += ans[parent[v]];
    }

    println!("{}", ans.iter().join(" "));
}
