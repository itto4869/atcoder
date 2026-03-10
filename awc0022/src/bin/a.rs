use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: u64,
        k: u64,
    }
    let mut ans = 0;
    for _ in 0..n {
        input! {
            w: u64,
        }
        let rem = w.saturating_sub(d * k);
        if rem >= 1 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
