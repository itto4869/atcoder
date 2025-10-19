use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut count = HashMap::new();
    for &ai in &a {
        *count.entry(ai).or_insert(0u64) += 1;
    }

    let mut res = 0;
    for x in 0..10u64.pow(5) {
        let left = if x == 0 { 0 } else { *count.get(&(x - 1)).unwrap_or(&0) };
        let mid = *count.get(&x).unwrap_or(&0);
        let right = *count.get(&(x + 1)).unwrap_or(&0);
        res = res.max(left + mid + right);
    }

    println!("{}", res);
}
