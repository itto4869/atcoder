use std::collections::HashSet;

use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Bytes,
    }
    let mut set = HashSet::new();
    for i in 0..n {
        set.insert(s[i]);
        if set.len() == 3 {
            println!("{}", i + 1);
            break;
        }
    }
}
