use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
    }
    let mut s = Vec::with_capacity(n);
    let mut c = Vec::with_capacity(n);
    let mut p = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            si: f64,
            ci: usize,
            pi: f64,
        }
        s.push(si);
        c.push(ci);
        p.push(pi);
    }

    let num_states = 1 << n;
    let mut dp = vec![vec![0.0; num_states]; x + 1];

    for cost in (0..x).rev() {
        for mask in 0..num_states {
            let mut max_expected_value = 0.0;

            for i in 0..n {
                if (mask >> i) & 1 != 0 {
                    continue;
                }

                let next_cost = cost + c[i];
                if next_cost > x {
                    continue;
                }

                let prob_success = p[i] / 100.0;
                let prob_fail = 1.0 - prob_success;

                let term_success = prob_success * (s[i] + dp[next_cost][mask | (1 << i)]);

                let term_fail = prob_fail * (dp[next_cost][mask]);

                let expected_value = term_success + term_fail;

                if max_expected_value < expected_value {
                    max_expected_value = expected_value;
                }

            }

            dp[cost][mask] = max_expected_value;
        }
    }

    println!("{}", dp[0][0]);
}
