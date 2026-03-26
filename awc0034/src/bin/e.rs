use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [i64; n],
        w: [i64; n - 1],
    }
    let mut dp = vec![vec![-1i64; n]; 1 << n];
    for i in 0..n {
        dp[1 << i][i] = 0;
    }

    for s in 0..(1 << n) {
        let cnt = (s as usize).count_ones() as usize;
        if cnt <= 1 {
            continue;
        }

        for j in 0..n {
            if (s >> j) & 1 == 0 {
                continue;
            }

            let prev_s = s ^ (1 << j);

            for k in 0..n {
                if (prev_s >> k) & 1 == 0 {
                    continue;
                }

                let add = (p[k] - p[j]).abs() * w[cnt - 2];
                dp[s][j] = dp[s][j].max(dp[prev_s][k] + add);
            }
        }
    }

    let ans = dp[(1 << n) - 1].iter().max().unwrap();
    println!("{}", ans);
}
