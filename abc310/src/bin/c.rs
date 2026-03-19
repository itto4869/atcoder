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
            mut s: String,
        }
        let rev_s = s.chars().rev().collect::<String>();
        
        let s = s.min(rev_s);

        set.insert(s);
    }

    let ans = set.len();
    println!("{}", ans);
}
