use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [u64; n],
    }
    let mut set = HashSet::new();
    let mut ans = 0;
    for i in 0..n {
        let ai = a[i];
        set.insert(ai);
        if set.len() == m {
            ans = n - i;
            break;
        }
    }

    println!("{}", ans);
}
