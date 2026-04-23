use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let mut ans = a / b;
    let r = a % b;
    if r * 2 > b {
        ans += 1;
    }

    println!("{}", ans);
}
