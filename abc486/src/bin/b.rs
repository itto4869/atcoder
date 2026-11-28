use std::collections::HashSet;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            a: usize,
        }
        set.insert(a);
    }

    for _ in 0..m {
        input! {
            x: usize,
            y: usize,
        }
        set.remove(&x);
        set.insert(y);
    }

    let mut ans = set.into_iter().collect_vec();
    ans.sort_unstable();
    println!("{}", ans.iter().format(" "));
}
