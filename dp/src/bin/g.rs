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

    let mut dp = vec![-1; n];
    let mut ans = 0;
    for v in 0..n {
        ans = ans.max(dfs(v, &graph, &mut dp));
    }

    println!("{}", ans);
}

fn dfs(v: usize, graph: &Vec<Vec<usize>>, dp: &mut Vec<i32>) -> i32 {
    if dp[v] != -1 {
        return dp[v];
    }

    let mut res = 0;
    for &u in &graph[v] {
        res = res.max(dfs(u, &graph, dp) + 1);
    }

    dp[v] = res;
    return res;
}