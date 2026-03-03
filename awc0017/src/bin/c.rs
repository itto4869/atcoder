use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        c: [u64; n],
    }
    let mut dp = vec![0; n];
    for i in 1..n {
        if c[i] == c[i - 1] {
            dp[i] = dp[i - 1] + 1;
        } else {
            dp[i] = dp[i - 1];
        }
    }

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
        }
        let res = dp[r] - dp[l];
        println!("{}", res);
    }
}
