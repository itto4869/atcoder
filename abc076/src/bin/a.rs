use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let ans = b * 2 - a;
    println!("{}", ans);
}
