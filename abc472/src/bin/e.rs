use proconio::{fastout, input, marker::Usize1};
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            edges: [(Usize1, Usize1); m],
        }

        let mut graph = vec![Vec::new(); n];

        for &(u, v) in &edges {
            graph[u].push(v);
            graph[v].push(u);
        }

        // BFS で全域木を作る
        let mut parent = vec![n; n];
        let mut depth = vec![0usize; n];
        let mut color = vec![0u8; n];

        let mut queue = VecDeque::new();

        parent[0] = 0;
        queue.push_back(0);

        while let Some(v) = queue.pop_front() {
            for &next in &graph[v] {
                if parent[next] != n {
                    continue;
                }

                parent[next] = v;
                depth[next] = depth[v] + 1;
                color[next] = color[v] ^ 1;

                queue.push_back(next);
            }
        }

        // 同色同士を結ぶ辺を探す
        let bad_edge = edges
            .iter()
            .copied()
            .find(|&(u, v)| color[u] == color[v]);

        let Some((mut u, mut v)) = bad_edge else {
            println!("-1");
            continue;
        };

        // 全域木上の u-v パスを復元する
        let mut left = Vec::new();
        let mut right = Vec::new();

        // 深さを揃える
        while depth[u] > depth[v] {
            left.push(u);
            u = parent[u];
        }

        while depth[v] > depth[u] {
            right.push(v);
            v = parent[v];
        }

        // LCA まで同時に上る
        while u != v {
            left.push(u);
            right.push(v);

            u = parent[u];
            v = parent[v];
        }

        // LCA
        left.push(u);

        // right は v -> LCA の順なので逆転
        right.reverse();
        left.extend(right);

        // 0-indexed -> 1-indexed
        let cycle: Vec<usize> = left.into_iter().map(|x| x + 1).collect();

        println!("{}", cycle.len());
        println!(
            "{}",
            cycle
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );
    }
}