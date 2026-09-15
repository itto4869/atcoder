use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            a: usize,
        }
        *map.entry(a).or_insert(0) += 1usize;
    }

    let mut ans = 0;
    for &v in map.values() {
        ans += v / 2;
    }

    println!("{}", ans);
}
