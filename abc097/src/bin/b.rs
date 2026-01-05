use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
    }
    let mut set = HashSet::new();
    set.insert(1);
    for i in 2..=40 {
        let mut n = i * i;
        while n <= 1000 {
            set.insert(n);
            n *= i;
        }
    }

    for i in (0..=x).rev() {
        if set.contains(&i) {
            println!("{}", i);
            break;
        }
    }
}