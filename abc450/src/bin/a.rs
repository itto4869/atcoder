use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let ans = (1..=n).rev();
    println!("{}", ans.format(","));
}
