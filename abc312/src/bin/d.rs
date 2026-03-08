use proconio::{fastout, input, marker::Bytes};

const MOD: u64 = 998244353;
#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    if s.len() % 2 == 1 {
        println!("0");
        return;
    }

    let mut dp = vec![vec![0; s.len() / 2 + 1]; s.len() + 1];
    dp[0][0] = 1;
    for i in 1..=s.len() {
        for j in ((i + 1) / 2)..=(s.len() / 2).min(i) {
            if s[i - 1] == b'(' {
                dp[i][j] = dp[i - 1][j - 1];
            } else if s[i - 1] == b')' {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j]) % MOD;
            }
        }
    }

    println!("{}", dp[s.len()][s.len() / 2]);
}
