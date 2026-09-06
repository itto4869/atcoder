use std::collections::HashSet;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        p: [usize; n],
        a: [usize; q],
    }
    let mut set = HashSet::new();
    let mut v = Vec::new();
    for &ai in a.iter().rev() {
        if set.contains(&ai) {
            continue;
        }

        v.push(ai);
        set.insert(ai);
    }

    for &i in p.iter().rev() {
        if set.contains(&i) {
            continue;
        }

        v.push(i);
    }

    v.reverse();
    println!("{}", v.iter().format(" "));
}
