use ac_library::{Mod998244353, StaticModInt};
use proconio::{fastout, input};

type M = StaticModInt<Mod998244353>;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut dp = vec![vec![M::new(0); 2]; n];
    dp[0][1] = M::new(m);
    for i in 1..n {
        dp[i][0] = dp[i][0] + dp[i - 1][0] * (m - 2) + dp[i - 1][1] * (m - 1);
        dp[i][1] = dp[i][1] + dp[i - 1][0];
    }

    println!("{}", dp[n - 1][0]);
}
