use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        l: usize,
        s: u64,
        t: u64,
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
            c: u64,
        }
        graph[u].push((v, c));
    }

    let mut queue = VecDeque::new();
    let mut ok = vec![false; n];
    queue.push_back((0, 0, 0));
    while let Some((v, sum_cost, cnt)) = queue.pop_front() {
        if cnt == l {
            if s <= sum_cost && sum_cost <= t {
                ok[v] = true;
            }
            continue;
        }

        let next = &graph[v];
        for &(u, cost) in next {
            queue.push_back((u, sum_cost + cost, cnt + 1));
        }
    }

    let mut ans = Vec::new();
    for i in 0..n {
        if ok[i] {
            ans.push(i + 1);
        }
    }

    println!("{}", ans.iter().format(" "));
}
