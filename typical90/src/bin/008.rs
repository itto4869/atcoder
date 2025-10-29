use proconio::{fastout, input, marker::Bytes};

const MOD: u64 = 1_000_000_007;
#[fastout]
fn main() {
    input! {
        n: usize,
        s: Bytes,
    }
    let mut dp = vec![vec![0; 8]; n+1];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..=7 {
            dp[i + 1][j] += dp[i][j];
            if s[i] == b'a' && j == 0 { dp[i + 1][j + 1] += dp[i][j] };
            if s[i] == b't' && j == 1 { dp[i + 1][j + 1] += dp[i][j] };
            if s[i] == b'c' && j == 2 { dp[i + 1][j + 1] += dp[i][j] };
            if s[i] == b'o' && j == 3 { dp[i + 1][j + 1] += dp[i][j] };
            if s[i] == b'd' && j == 4 { dp[i + 1][j + 1] += dp[i][j] };
            if s[i] == b'e' && j == 5 { dp[i + 1][j + 1] += dp[i][j] };
            if s[i] == b'r' && j == 6 { dp[i + 1][j + 1] += dp[i][j] };
        }
        for k in 0..=7 {
            dp[i + 1][k] %= MOD;
        }
    }

    println!("{}", dp[n][7])
}
