use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }
    let ans = a.min(b) + c.min(d);
    println!("{}", ans);
}
