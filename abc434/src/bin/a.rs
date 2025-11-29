use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        w: u64,
        b: u64,
    }
    let ans = (w * 1000) / b + 1;
    println!("{}", ans);
}
