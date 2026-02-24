use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: u64,
        h: [u64; n],
        c: [u64; n]
    }
    let mut ans = 0;
    for i in 0..n {
        if h[i] <= t {
            ans += c[i];
        }
    }

    println!("{}", ans);
}
