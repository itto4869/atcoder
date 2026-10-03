use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut m: usize,
    }
    let mut v = vec![0; n];
    let mut idx = 0;
    while m > 0 {
        v[idx] += 1;
        m -= 1;
        idx = (idx + 1) % n;
    }

    println!("{}", v.iter().format("\n"));
}
