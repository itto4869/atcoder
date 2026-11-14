use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        b: usize,
        r: usize,
    }
    let ans = (30 - b) * (30 - r);
    println!("{}", ans);
}
