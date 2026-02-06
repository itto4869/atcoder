use cp_library::math::combinations::{self, Combination};
use proconio::{fastout, input};

const MOD: u64 = 998244353;
#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n],
    }
    let comb = Combination::new(k + 1, MOD);

    let mut dp = vec![vec![0; k + 1]; 3];

    dp[0][0] = 1;

    for &val in &a {
        for s in 0..=k {
            dp[1][s] = (dp[1][s] + dp[0][s]) % MOD;
        }

        for s in 0..=k {
            dp[2][s] = (dp[2][s] + dp[1][s]) % MOD;
        }

        let mut next_dp = vec![vec![0; k + 1]; 3];

        for j in 0..3 {
            for s in 0..=k {
                if dp[j][s] == 0 { continue; }

                next_dp[j][s] = (next_dp[j][s] + dp[j][s]) % MOD;

                if j == 1 {
                    let mut val_pow = 1;
                    for p in 1..=(k - s) {
                        val_pow = (val_pow * val) % MOD;

                        let ways = dp[j][s] * comb.n_c_r(k - s, p) * val_pow % MOD;

                        next_dp[j][s + p] = (next_dp[j][s + p] + ways) % MOD;
                    }
                }
            }
        }
        dp = next_dp;
    }

    for s in 0..=k {
        dp[1][s] = (dp[1][s] + dp[0][s]) % MOD;
    }

    for s in 0..=k {
        dp[2][s] = (dp[2][s] + dp[1][s]) % MOD;
    }

    println!("{}", dp[2][k]);
}
