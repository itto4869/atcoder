use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let ans = 12 * a + b;
    println!("{}", ans);
}
