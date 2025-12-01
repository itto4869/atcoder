use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let ans = n / 3;
    println!("{}", ans);
}
