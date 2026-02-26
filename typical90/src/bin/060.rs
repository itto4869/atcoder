use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [u64; n],
    }
    let mut up_dp = vec![u64::MAX; n];
    let mut up_v = vec![0; n];
    for i in 0..n {
        let ai = a[i];
        let idx = up_dp.partition_point(|&x| x < ai);
        
        up_v[i] = idx + 1;
        up_dp[idx] = ai;
    }

    let mut down_dp = vec![u64::MAX; n];
    let mut down_v = vec![0; n];
    a.reverse();
    for i in 0..n {
        let ai = a[i];
        let idx = down_dp.partition_point(|&x| x < ai);

        down_v[n - i - 1] = idx;
        down_dp[idx] = ai;
    }

    let mut ans = 0;
    for i in 0..n {
        ans = ans.max(up_v[i] + down_v[i]);
    }

    println!("{}", ans);
}
