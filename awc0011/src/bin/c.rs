use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
    }
    let mut ans = 0;
    let mut res = 0;
    for _ in 0..n {
        input! {
            a: u64,
        }
        if a | k == k {
            res |= a;
            ans += 1;
        }
    }

    if ans > 0 && res == k {
        println!("{}", ans);
    } else {
        println!("-1");
    }
}
