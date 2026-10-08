use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..(n - 1) {
        input! {
            a: Usize1,
            b: Usize1
        }
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut queue = VecDeque::new();
    queue.push_front(0);
    let mut seen = vec![false; n];
    seen[0] = true;
    let mut vv = vec![0; n];
    vv[0] = 1;
    let mut ans = Vec::new();
    ans.push(1);
    while let Some(v) = queue.pop_front() {
        let next = &graph[v];
        for &u in next {
            if seen[u] {
                continue;
            }

            seen[u] = true;
            if vv[v] == 0 {
                vv[u] = 1;
                ans.push(u + 1);
            }

            queue.push_back(u);
        }
    }

    if ans.len() < (n / 2) {
        let mut queue = VecDeque::new();
        queue.push_front(0);
        let mut seen = vec![false; n];
        seen[0] = true;
        let mut vv = vec![0; n];
        let mut ans = Vec::new();
        while let Some(v) = queue.pop_front() {
            let next = &graph[v];
            for &u in next {
                if seen[u] {
                    continue;
                }

                seen[u] = true;
                if vv[v] == 0 {
                    vv[u] = 1;
                    ans.push(u + 1);
                }

                queue.push_back(u);
            }
        }

        let mut v = Vec::new();
        for i in 0..(n / 2) {
            v.push(ans[i]);
        }

        println!("{}", v.iter().format(" "));
        return;
    }
    let mut v = Vec::new();
    for i in 0..(n / 2) {
        v.push(ans[i]);
    }

    println!("{}", v.iter().format(" "));
}
