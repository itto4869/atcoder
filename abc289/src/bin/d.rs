use std::collections::HashSet;

use cp_utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize,
    }
    let set: HashSet<usize> = HashSet::from_iter(b);
    let mut dp = vec![false; x + 100001];
    dp[0] = true;
    for i in 0..=x {
        if !dp[i] {
            continue;
        }

        for j in 0..n {
            let next = i + a[j];
            if set.contains(&next) {
                continue;
            } else {
                dp[next] = true;
            }
        }
    }

    yes_no!(dp[x]);
}
