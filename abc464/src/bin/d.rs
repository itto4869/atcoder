use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            s: Chars,
            x: [i64; n],
            y: [i64; n - 1],
        }
        let mut v = Vec::with_capacity(n);
        for i in 0..(n - 1) {
            let mut d = y[i];
            if s[i] == 'S' {
                d -= x[i];
            }

            if s[i + 1] == 'R' {
                d -= x[i + 1];
            }

            v.push(d);
        }

        if n == 2 {
            println!("{}", v[0].max(0));
            continue;
        }
        let mut dp = vec![0; n];
        dp[0] = v[0].max(0);
        dp[1] = dp[0].max(v[1]);

        for i in 2..(n - 1) {
            dp[i] = dp[i - 1].max(dp[i - 2] + v[i]);
        }

        let ans = dp[n - 2];
        println!("{}", ans);
    }
}
