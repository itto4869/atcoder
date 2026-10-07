use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s: Chars,
    }
    for i in (0..s.len()).step_by(2) {
        s.swap(i, i + 1);
    }

    println!("{}", s.iter().format(""));
}
