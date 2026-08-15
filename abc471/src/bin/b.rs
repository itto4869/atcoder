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
            s: String
        }
        let s = s.to_ascii_lowercase();
        *map.entry(s).or_insert(0) += 1;
    }

    let &ans = map.values().max().unwrap();
    println!("{}", ans);
}
