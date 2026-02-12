use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut h: [i64; n],
    }
    h.sort_unstable();
    let mut ans = 0;
    ans += h[0].abs();
    for i in 1..n {
        ans += (h[i] - h[i - 1]).abs();
    }

    ans += h[n - 1].abs();
    println!("{}", ans);
}
