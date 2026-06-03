use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }
    a.sort_unstable();
    println!("{}", a.iter().format(" "));
}
