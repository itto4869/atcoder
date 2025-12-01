use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            s: char,
        }
        set.insert(s);
    }
    if set.len() == 3 {
        println!("Three");
    } else {
        println!("Four");
    }
}
