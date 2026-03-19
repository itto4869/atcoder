use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: u64,
        b: u64,
        k: u64,
    }
    let mut ans = 0;
    for _ in 0..n {
        input! {
            c: u64,
        }
        if c >= k {
            ans += c * (p + b);
        } else {
            ans += c * p;
        }
    }

    println!("{}", ans);
}
