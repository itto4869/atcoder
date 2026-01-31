use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize; n],
    }
    let mut open_t = 0;
    let mut ans = 0;
    for i in 0..n {
        let ai = a[i];
        if ai > open_t {
            ans += ai - open_t;
            open_t = ai + 100;
        }
    }

    if open_t < t {
        ans += t - open_t;
    }
    println!("{}", ans);
}
