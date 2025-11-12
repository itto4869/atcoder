use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let ans = (a + b + 1) / 2;
    println!("{}", ans);
}
