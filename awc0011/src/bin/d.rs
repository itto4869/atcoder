use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        v: [u64; n],
        p: [Usize1; n - 1],
    }

    let mut dp = vec![0; n];
    dp[0] = v[0];
    for i in 1..n {
        let j = p[i - 1];
        dp[i] = dp[j] + v[i];
    }

    for _ in 0..q {
        input! {
            x: Usize1,
        }
        println!("{}", dp[x]);
    }
}
