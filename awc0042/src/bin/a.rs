use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
    }
    let mut ans = 0;
    for _ in 0..n {
        input! {
            a: u64,
            b: u64,
        }
        ans += (a + b + k - 1) / k;
    }

    println!("{}", ans);
}
