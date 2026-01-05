use std::collections::HashSet;

use proconio::{fastout, input, marker::{Bytes, Chars}};

#[fastout]
fn main() {
    input! {
        s: Chars,
        k: usize,
    }
    let mut set = HashSet::new();
    for i in 0..s.len() {
        for j in 1..=k {
            let sub: String = s[i..(i + j).min(s.len())].iter().collect();
            set.insert(sub);
        }
    }

    let mut set_vec: Vec<&String> = set.iter().collect();
    set_vec.sort();
    println!("{}", set_vec[k - 1]);
}
