use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut map = HashMap::new();
    let mut imos = vec![0; n];
    imos[0] = a[0];
    for i in 1..n {
        imos[i] = imos[i - 1] + a[i];
    }

    for i in 0..n {
        let ai = imos[i];
        map.entry(ai % k).or_insert(Vec::new()).push(i);
    }
    let mut dp = vec![0; n + 1];
    for i in 1..=n {
        let ai = imos[i - 1] % k;
        if let Some(v) = map.get(&ai) {
            let bidx = v.partition_point(|&x| x < (i - 1));
            if bidx == 0 {
                if ai == 0 {
                    dp[i] = dp[i - 1].max(1);
                } else {
                    dp[i] = dp[i - 1];
                }
            } else {
                let idx = v[bidx - 1] + 1;
                dp[i] = dp[i - 1].max(dp[idx] + 1);
            }
        } else {
            unreachable!();
        }
    }

    println!("{}", dp[n]);
}
