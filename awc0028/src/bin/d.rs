use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut graph = vec![Vec::new(); n];
    let mut back_graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1
        }
        graph[a].push(b);
        back_graph[b].push(a);
    }
    let mut heap = BinaryHeap::new();
    let mut seen = vec![false; n];
    let mut cnts = vec![0; n];
    for i in 0..n {
        cnts[i] = back_graph[i].len();
        if back_graph[i].is_empty() {
            heap.push(Reverse(i));
        }
    }

    let mut ans = Vec::with_capacity(n);
    let mut seen = vec![false; n];
    while let Some(v) = heap.pop() {
        let v = v.0;
        ans.push(v + 1);
        let next = &graph[v];
        for &u in next {
            cnts[u] -= 1;
            if cnts[u] == 0 {
                heap.push(Reverse(u));
            }
        }
    }

    println!("{}", ans.iter().format(" "));
}
