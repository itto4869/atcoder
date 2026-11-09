use ac_library::{Segtree, Min};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        c: [[usize; k]; n],
    }
    let mut dp = Vec::with_capacity(n + 1);
    for _ in 0..=n {
        let seg = Segtree::<Min<usize>>::new(k);
        dp.push(seg);
    }

    for i in 0..k {
        dp[0].set(i, 0);
    }

    for i in 1..=n {
        for j in 0..k {
            let min_c = dp[i - 1].prod(0..j).min(dp[i - 1].prod((j + 1)..k));
            dp[i].set(j, min_c + c[i - 1][j]);
        }
    }

    let ans = dp[n].all_prod();

    println!("{}", ans);
}
