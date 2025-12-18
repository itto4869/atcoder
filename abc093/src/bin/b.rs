use std::collections::HashSet;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        k: usize,
    }
    let mut set = HashSet::new();
    for i in a..((a + k).min(b)) {
        set.insert(i);
    }

    for i in ((b.saturating_sub(k) + 1).max(a))..=b {
        set.insert(i);
    }

    let mut ans: Vec<usize> = set.into_iter().collect();
    ans.sort();
    println!("{}", ans.iter().format("\n"));
}
