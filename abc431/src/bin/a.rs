use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: u64,
        b: u64,
    }
    let ans = h.saturating_sub(b);
    println!("{}", ans);
}
