use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        xyz: [(u64, u64, usize); n],
    }
    let sum_z = xyz.iter().map(|(_, _, z)| z).sum();
    let mut dp = vec![1 << 60; sum_z + 1];
    dp[0] = 0;
    for &(x, y, z) in &xyz {

        let w = (y.saturating_sub(x) + 1) / 2;
        for j in (z..=sum_z).rev() {
            dp[j] = dp[j].min(dp[j - z] + w);
        }
    }

    let mut ans = 1 << 60;
    for j in (sum_z / 2 + 1)..=sum_z {
        ans = ans.min(dp[j]);
    }

    println!("{}", ans);
}
