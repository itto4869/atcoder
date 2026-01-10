use std::collections::{HashMap, HashSet};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u64; n],
    }
    a.sort_unstable();

    for _ in 0..q {
        input! {
            x: u64,
            y: u64,
        }

        let mut ok = 1 << 60;
        let mut ng = x - 1;

        let count_less_x = a.partition_point(|&v| v < x);

        while ok - ng > 1 {
            let mid = ng + (ok - ng) / 2;

            let total_nums = mid - x + 1;

            let count_le_mid = a.partition_point(|&v| v <= mid);

            let present_in_range = count_le_mid - count_less_x;

            let missing_count = total_nums - present_in_range as u64;

            if missing_count >= y {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        println!("{}", ok);
    }
}
