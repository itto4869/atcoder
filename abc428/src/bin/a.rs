use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s: u64,
        a: u64,
        b: u64,
        x: u64,
    }
    let mut ans = 0;
    let mut state = 0;
    let mut t = 0;
    while t < x {
        if state == 0 {
            t = t + a;
            if t > x {
                ans += s * (a - (t - x));
            } else {
                ans += s * a;
            }
            state = 1;
        } else {
            t = t + b;
            state = 0;
        }
    }
    println!("{}", ans);
}
