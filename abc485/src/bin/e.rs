use std::collections::VecDeque;

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
    }

    let mut queue = VecDeque::new();
    queue.push_back((0, 0, 0));

    // 各都市で探索済みの直前の色（最大2種類）
    let mut used = vec![Vec::<usize>::new(); n];

    while let Some((v, c, d)) = queue.pop_front() {
        // 同じ色で探索済み、または2色探索済みなら無視
        if used[v].contains(&c) || used[v].len() == 2 {
            continue;
        }

        used[v].push(c);

        if v == n - 1 {
            println!("{}", d);
            return;
        }

        for &(nv, nc) in &graph[v] {
            if nc != c {
                queue.push_back((nv, nc, d + 1));
            }
        }
    }

    println!("-1");
}