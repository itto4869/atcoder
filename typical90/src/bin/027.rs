use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut set = HashSet::new();
    for i in 1..=n {
        input! {
            s: String,
        }
        if set.insert(s) {
            println!("{}", i);
        }
    }
}
