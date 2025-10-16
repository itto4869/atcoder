use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }
    let ans = b.min(d).saturating_sub(a.max(c));
    println!("{}", ans);
}
