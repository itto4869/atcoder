use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        x: u128,
        y: u128,
    }
    let mut n = y / x;
    let mut ans = 0;
    while n > 0 {
        n /= 2;
        ans += 1;
    }
    println!("{}", ans);
}
