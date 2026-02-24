use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: u64,
        c: u64,
        d: u64,
    }
    let mut ans = 0;
    for _ in 0..n {
        input! {
            w: u64,
        }
        if w >= t && d < c {
            ans += d;
        } else if w >= t && d >= c {
            ans += c;
        }
    }

    println!("{}", ans);
}
