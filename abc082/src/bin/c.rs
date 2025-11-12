use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut cnt = HashMap::new();
    for &ai in &a {
        *cnt.entry(ai).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (k, v) in cnt {
        if k <= v {
            ans += v - k;
        } else {
            ans += v;
        }
    }
    println!("{}", ans);
}
