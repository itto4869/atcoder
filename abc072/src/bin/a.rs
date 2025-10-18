use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u64,
        t: u64,
    }
    let ans = x.saturating_sub(t);
    println!("{ans}");
}
