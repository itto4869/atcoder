use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64
    }
    let ans = (a * n).min(b);
    println!("{}", ans);
}
