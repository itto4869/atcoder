use std::collections::HashSet;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut set = HashSet::new();
    for c in s {
        set.insert(c);
    }

    for c in 'a'..='z' {
        if !set.contains(&c) {
            println!("{}", c);
            return;
        }
    }
}
