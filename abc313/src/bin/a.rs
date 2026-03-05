use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p1: i64,
    }
    let mut max_p = 0;
    for _ in 0..(n - 1) {
        input! {
            p: i64,
        }
        max_p = max_p.max(p);
    }

    let ans = (max_p - p1).max(-1) + 1;
    println!("{}", ans);
}
