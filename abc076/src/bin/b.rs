use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
    }
    let mut ans = 1;
    for _ in 0..n {
        if ans < k {
            ans *= 2;
        } else {
            ans += k;
        }
    }
    println!("{}", ans);
}
