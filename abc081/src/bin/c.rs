use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n],
    }
    let mut cnt = HashMap::new();
    for &ai in &a {
        *cnt.entry(ai).or_insert(0u64) += 1;
    }
    let mut ans = 0;
    let mut cnt: Vec<(&u64, &u64)> = cnt.iter().collect();
    cnt.sort_by(|&a, &b| a.1.cmp(b.1));
    for i in 0..(cnt.len().saturating_sub(k)) {
        ans += cnt[i].1;
    }
    println!("{}", ans);
}