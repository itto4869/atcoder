use proconio::{fastout, input};

const MOD: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        k: usize,
    }
    if k % 9 != 0 {
        println!("0");
    } else {
        let mut dp = vec![0; k + 1];
        dp[0] = 1;
        for i in 1..=k {
            let b = i.min(9);
            for j in 1..=b {
                dp[i] = (dp[i] + dp[i - j]) % MOD;
            }
        }
        println!("{}", dp[k]);
    }
}
