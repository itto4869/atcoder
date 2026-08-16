use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        l: u128,
    }
    let mut ans = 1;
    for x in (l - 1 - 10)..=(l - 1) {
        ans *= x;
    }

    for x in 1..=11 {
        ans /= x;
    }
    println!("{}", ans);
}
