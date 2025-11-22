use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        value: [[u64; n]; 2],
    }
    let mut dp = vec![vec![0; n + 1]; 2];
    for i in 0..n {
        dp[0][i + 1] = dp[0][i] + value[0][i];
        dp[1][i + 1] = dp[1][i] + value[1][i];
    }
    
    let mut ans = 0;
    for i in 1..=n {
        let v = dp[0][i] +  dp[1][n] - dp[1][i - 1];
        ans = ans.max(v);
    }
    println!("{}", ans);
}
