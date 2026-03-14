use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: Bytes,
    }
    let mut dp = vec![vec![0; 26]; n];
    dp[0][(s[0] - b'a') as usize] += 1;
    for i in 1..n {
        for j in 0..26 {
            if (s[i] - b'a') as usize == j {
                dp[i][j] = dp[i - 1][j] + 1;
            } else {
                dp[i][j] = dp[i - 1][j];
            }
        }
    }

    let mut ans = 0u64;
    for i in l..n {
        if i > r {
            ans += dp[i - l][(s[i] - b'a') as usize] - dp[i - r - 1][(s[i] - b'a') as usize];
        } else {
            ans += dp[i - l][(s[i] - b'a') as usize];
        }
    }

    println!("{}", ans);
}
