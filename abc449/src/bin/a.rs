use std::f64::consts::PI;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: f64,
    }
    let ans = (d / 2.0) * (d / 2.0) * PI;
    println!("{}", ans);
}
