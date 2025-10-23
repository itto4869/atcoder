use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
        a: u64,
    }
    let ans = n * n - a;
    println!("{}", ans);
}
