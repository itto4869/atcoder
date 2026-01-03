use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u32,
    }
    let ans = 2u64.pow(n) - 2 * n as u64;
    println!("{}", ans);
}
