use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        x: [u64; n],
    }
    let mut ans = 0;
    for &xi in &x {
        ans += xi.min(xi.abs_diff(k));
    }
    println!("{}", ans * 2);
}
