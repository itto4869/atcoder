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
    

    let mut seen = vec![false; n];
    let mut dp = vec![0; n];
    seen[0] = true;
    dfs(0, &graph, &mut dp, &mut seen);

    let mut seen = vec![false; n];
    seen[0] = true;
    let mut ans = 0;
    dfs2(0, &graph, &dp, n, &mut ans, &mut seen);
    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<usize>, seen: &mut Vec<bool>) {
    let next = &graph[v];
    dp[v] = 1;
    for &u in next {
        if seen[u] {
            continue;
        }

        seen[u] = true;
        dfs(u, graph, dp, seen);

        dp[v] += dp[u];
    }
}

fn dfs2(v: usize, graph: &Vec<Vec<usize>>, dp: &Vec<usize>, n: usize, ans: &mut usize, seen: &mut Vec<bool>) {
    let next = &graph[v];
    for &u in next {
        if seen[u] {
            continue;
        }

        *ans += (n - dp[u]) * dp[u];
        seen[u] = true;
        dfs2(u, graph, dp, n, ans, seen);
    }
}