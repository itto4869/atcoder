use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        c: u64,
        w: [u64; n],
    }
    let inf = (n + 1, 0u64);
    let mut dp = vec![inf; 1 << n];
    dp[0] = (1usize, 0u64);

    for mask in 0..(1 << n) {
        let (rides, last_weight) = dp[mask];
        for i in 0..n {
            if (mask >> i) & 1 == 1 {
                continue;
            }

            let next = if last_weight + w[i] <= c {
                (rides, last_weight + w[i])
            } else {
                (rides + 1, w[i])
            };

            let nmask = mask | (1 << i);
            if next < dp[nmask] {
                dp[nmask] = next;
            }
        }
    }

    println!("{}", dp[(1 << n) - 1].0);
}
