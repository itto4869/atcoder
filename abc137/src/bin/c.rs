use std::collections::HashMap;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            mut s: Chars,
        }
        s.sort_unstable();
        *map.entry(s).or_insert(0) += 1usize;
    }

    let mut ans = 0;
    for &v in map.values() {
        ans += ((v * (v - 1)) / 2);
    }

    println!("{}", ans);
}
