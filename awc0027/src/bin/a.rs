use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: u64,
        t: u64,
    }
    let mut ans = 0;
    for _ in 0..n {
        input! {
            a: u64,
        }
        if a.abs_diff(s) <= t {
            ans += 1;
        }
    }

    println!("{}", ans);
}
