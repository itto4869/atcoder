use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut rc: [(usize, usize); m],
    }
    let mut r_set = HashSet::new();
    let mut c_set = HashSet::new();

    let mut set = HashSet::new();
    for &(r, c) in rc.iter().rev() {
        if r_set.contains(&r) || c_set.contains(&c) {
        } else {
            set.insert((r, c));
        }

        r_set.insert(r);
        c_set.insert(c);
    }

    println!("{}", set.len());
}
