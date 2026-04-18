use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: u64,
        r: u64,
    }
    println!("{}", r - l + 1);
}
