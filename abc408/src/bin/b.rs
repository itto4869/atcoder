use std::collections::HashSet;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            a: u64,
        }
        set.insert(a);
    }

    let mut ans: Vec<u64> = set.into_iter().collect();
    ans.sort();
    println!("{}\n{}", ans.len(), ans.iter().format(" "));
}
