use std::thread::current;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u8; n],
    }
    let mut dp = vec![vec![vec![-1.0; n + 1]; n + 1]; n + 1];
    let mut one = 0;
    let mut two = 0;
    let mut three = 0;
    for &ai in &a {
        if ai == 1 {
            one += 1;
        } else if ai == 2 {
            two += 1;
        } else {
            three += 1;
        }
    }

    dp[0][0][0] = 0.0;

    fn get_expect(k: usize, j: usize, i: usize, n: usize, dp: &mut Vec<Vec<Vec<f64>>>) -> f64 {
        if dp[k][j][i] >= 0.0 {
            return dp[k][j][i]
        }

        let current_sum = i + j + k;
        if current_sum == 0 {
            return 0.0
        }

        let mut val = n as f64 / current_sum as f64;

        if k > 0 {
            val += (k as f64 / current_sum as f64) * get_expect(k - 1, j + 1, i, n, dp);
        }

        if j > 0 {
            val += (j as f64 / current_sum as f64) * get_expect(k, j - 1, i + 1, n, dp);
        }

        if i > 0 {
            val += (i as f64 / current_sum as f64) * get_expect(k, j, i - 1, n, dp);
        }

        dp[k][j][i] = val;
        val
    }

    let ans = get_expect(three, two, one, n, &mut dp);
    println!("{}", ans);
}
