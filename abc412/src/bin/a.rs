use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ans = 0;
    for _ in 0..n {
        input! {
            a: u64,
            b: u64,
        }
        if a < b {
            ans += 1;
        }
    }

    println!("{}", ans);
}
