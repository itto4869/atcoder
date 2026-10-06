use num::Integer;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }
    let x = a.gcd(&b).gcd(&c);
    let ans = (a - x) / x + (b - x) / x + (c - x) / x;
    println!("{}", ans);
}
