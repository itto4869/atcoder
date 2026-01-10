use std::collections::HashSet;

use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Bytes,
    }
    let mut ans = 0;
    let mut l_set = HashSet::new();
    let mut r_set = HashSet::new();
    for i in 0..n {
        for j in 0..i {
            l_set.insert(s[j]);
        }

        for j in i..n {
            r_set.insert(s[j]);
        }

        let set = l_set.intersection(&r_set);
        ans = ans.max(set.count());
        l_set.clear();
        r_set.clear();
    }

    println!("{}", ans);
}
