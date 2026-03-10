use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u64,
        t: u64,
    }
    let mut ans = 0;
    for _ in 0..n {
        input! {
            a: u64,
        }
        ans += t.saturating_sub(a);
    }
    if ans > m {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
