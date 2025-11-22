use std::u64;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    for i in 0..n {
        let ai = a[i];
        let mut idx = -1isize;
        for j in (0..i).rev() {
            if ai < a[j] {
                idx = j as isize + 1;
                break;
            }
        }
        println!("{}", idx);
    }
}
