use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: [String; n],
    }
    let mut s = s[..k].to_vec();
    s.sort_unstable();
    println!("{}", s.iter().format("\n"));
}
