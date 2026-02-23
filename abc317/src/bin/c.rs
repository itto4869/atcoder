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
            c: u64,
        }
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut ans = 0;
    for i in 0..n {
        dfs(i, &graph, 1 << i, 0, &mut ans);
    }

    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<(usize, u64)>>, seen: u64, dist: u64, ans: &mut u64) {
    let next = &graph[v];
    for &(u, c) in next {
        if seen & (1 << u) != 0 {
            continue;
        }

        *ans = *ans.max(&mut (dist + c));
        dfs(u, graph, seen | (1 << u), dist + c, ans);
    }
}