use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut set = HashSet::new();
    for i in 0..n {
        input! {
            s: String,
        }
        if set.contains(&s) {
            continue;
        } else {
            set.insert(s);
            println!("{}", i + 1);
        }
    }
}
