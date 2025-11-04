use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
        y: u64,
        z: u64,
    }
    let ans = (x - z) / (y + z);
    println!("{}", ans);
}
