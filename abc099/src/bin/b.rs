use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let ans = ((b - a - 1) * (b - a)) / 2 - a;
    println!("{}", ans);
}
