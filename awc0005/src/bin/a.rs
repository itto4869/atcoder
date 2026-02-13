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
            p: u64,
        }
        if p % k == 0 {
            ans += p;
        }
    }

    println!("{}", ans);
}
