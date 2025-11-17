use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut cnt = HashSet::new();
    for _ in 0..n {
        input! {
            d: u64,
        }
        cnt.insert(d);
    }
    println!("{}", cnt.len());
}
