use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        m: u64,
    }
    let ans = 24 + 24 - m;
    println!("{}", ans);
}
