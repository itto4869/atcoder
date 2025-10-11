use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        m: u64,
    }
    let ans = (n - 1) * (m - 1);
    println!("{}", ans);
}
