use proconio::{fastout, input};

const INF: i64 = 1 << 60;
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut dp = vec![vec![-INF; n + 1]; n + 1];
    let ans = dfs(0, n, &a, &mut dp);
    println!("{}", ans);
}

fn dfs(l: usize, r: usize, a: &Vec<i64>, dp: &mut Vec<Vec<i64>>) -> i64 {
    if dp[l][r] != -INF {
        return dp[l][r];
    }

    if l == r {
        return 0;
    }

    let mut res = -INF;
    res = res.max(-dfs(l + 1, r, a, dp) + a[l]);
    res = res.max(-dfs(l, r - 1, a, dp) + a[r - 1]);

    dp[l][r] = res;
    res
}