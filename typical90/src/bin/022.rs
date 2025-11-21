use std::mem::swap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    }
    let r = gcd(a, gcd(b, c));
    println!("{}", (a / r - 1) + (b / r - 1) + (c / r - 1));
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut a = x.max(y);
    let mut b = x.min(y);
    while a > 0 && b > 0 {
        a = a % b;
        if a < b {
            swap(&mut a, &mut b);
        }
    }
    a
}