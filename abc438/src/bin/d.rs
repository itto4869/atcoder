use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
        c: [u64; n],
    }
    let mut dp = vec![vec![0; 3]; n + 1];
    dp[0][0] = a[0];
    dp[0][1] = 0;
    dp[0][2] = 0;
    for i in 1..n {
        dp[i][0] = dp[i - 1][0] + a[i];
        dp[i][1] = (dp[i - 1][0] + b[i]).max(dp[i - 1][1] + b[i]);
        if i > 1 {
        
        dp[i][2] = (dp[i - 1][1] + c[i]).max(dp[i - 1][2] + c[i]);
        }
    }


    let ans = dp[n - 1][2];
    println!("{}", ans);
}
