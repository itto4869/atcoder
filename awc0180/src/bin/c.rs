use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = Vec::new();
    for i in 1..=n {
        input! {
            a: usize,
        }
        v.push((a, i));
    }

    v.sort_unstable();
    let v = v.into_iter().map(|(_, idx)| idx);
    println!("{}", v.format(" "));
}
