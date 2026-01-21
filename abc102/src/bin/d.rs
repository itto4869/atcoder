use std::i64;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut s = vec![0; n + 1];
    for i in 0..n {
        s[i + 1] = s[i] + a[i];
    }

    let total_sum = s[n];

    let mut min_diff = i64::MAX;

    for j in 2..=(n - 2) {
        let left_sum = s[j];
        let target_p = left_sum / 2;

        let i_idx = match s[0..=j].binary_search(&target_p) {
            Ok(idx) => idx,
            Err(idx) => idx
        };

        let mut best_p = -1;
        let mut best_q = -1;
        let mut current_left_diff = i64::MAX;

        for i in i_idx.saturating_sub(1)..=i_idx {
            if i > 0 && i < j {
                let p = s[i];
                let q = left_sum - p;
                if (p - q).abs() < current_left_diff {
                    current_left_diff = (p - q).abs();
                    best_p = p;
                    best_q = q;
                }
            }
        }

        let right_sum = total_sum - s[j];
        let target_r_sum = s[j] + right_sum / 2;

        let k_idx = match s[j..=n].binary_search(&target_r_sum) {
            Ok(idx) => idx + j,
            Err(idx) => idx + j,
        };

        let mut best_r = -1;
        let mut best_s_val = -1;
        let mut current_right_diff = i64::MAX;

        for k in k_idx.saturating_sub(1)..=k_idx {
            if k > j && k < n {
                let r = s[k] - s[j];
                let s_val = total_sum - s[k];
                if (r - s_val).abs() < current_right_diff {
                    current_right_diff = (r - s_val).abs();
                    best_r = r;
                    best_s_val = s_val;
                }
            }
        }

        let vals = vec![best_p, best_q, best_r, best_s_val];
        let max_val = vals.iter().max().unwrap();
        let min_val = vals.iter().min().unwrap();
        
        min_diff = min_diff.min(max_val - min_val);
    }

    println!("{}", min_diff);
}
