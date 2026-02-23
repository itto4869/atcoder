use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
    }
    let mut ans = 0;
    for _ in 0..m {
        input! {
            p: usize,
            v: u64,
        }
        if s < t {
            if s <= p && p <= t {
                ans += v;
            }
        } else {
            if t <= p && p <= s {
                ans += v;
            }
        }
    }

    println!("{}", ans);
}
