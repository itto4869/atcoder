use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut curr = 0;
    for _ in 0..(n - 1) {
        input! {
            a: u64,
            b: u64,
        }
        curr += a;
        curr = curr.saturating_sub(b);
    }

    input! {
        a: u64,
    }
    let ans = curr + a;
    println!("{}", ans);
}
