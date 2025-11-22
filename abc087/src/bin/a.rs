use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
        a: u64,
        b: u64,
    }
    let ans = (x - a) % b;
    println!("{}", ans);
}
