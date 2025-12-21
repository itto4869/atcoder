use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        x: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..m {
        input! {
            a: usize,
        }
        set.insert(a);
    }

    let mut left = 0;
    for i in 0..x {
        if set.contains(&i) {
            left += 1;
        }
    }

    let mut right = 0;
    for i in x..n {
        if set.contains(&i) {
            right += 1;
        }
    }

    println!("{}", left.min(right));
}
