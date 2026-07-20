use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans1 = 10000;
    let mut ans2 = 10000;
    for _ in 0..n {
        input! {
            a: usize,
            b: usize,
            s: String,
        }
        ans1 -= a;
        if s == "keep" {
            ans2 -= b;
        } else {
            ans2 -= a;
        }
    }

    let ans = ans1 - ans2;
    println!("{}", ans);
}
