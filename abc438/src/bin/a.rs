use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        d: u64,
        f: u64,
    }
    let rest = d - f;
    let ans = 7 - rest % 7;
    println!("{}", ans);
}
