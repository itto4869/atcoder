use ac_library::{Max, Segtree};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        w: usize,
        n: usize,
        lrv: [(usize, usize, i64); n],
    }
    let mut dp = Vec::with_capacity(n + 1);
    for _ in 0..=n {
        let segtree = Segtree::<Max<i64>>::new(w + 1);
        dp.push(segtree);
    }

    dp[0].set(0, 0);
    for i in 1..=n {
        let (l, r, v) = lrv[i - 1];
        for wi in 0..=w {
            if wi < l {
                let max_v = dp[i - 1].get(wi);
                dp[i].set(wi, max_v);
            } else {
                let max_v = dp[i - 1].get(wi).max(dp[i - 1].prod((wi.saturating_sub(r))..=(wi - l)) + v);
                dp[i].set(wi, max_v);
            }
        }
    }

    let ans = dp[n].get(w).max(-1);
    println!("{}", ans);
}
