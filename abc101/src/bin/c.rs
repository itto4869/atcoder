use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n],
    }
    let rest = n - k;
    let ans = 1 + (rest + k - 2) / (k - 1);
    println!("{}", ans);
}
