use std::collections::HashSet;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut concat_s = String::with_capacity(20);
    let mut set = HashSet::new();
    for ss in s.iter().permutations(2) {
        concat_s.push_str(ss[0]);
        concat_s.push_str(ss[1]);
        set.insert(concat_s.clone());
        concat_s.clear();
    }
    println!("{}", set.len());
}
