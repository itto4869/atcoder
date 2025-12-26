use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        x: u64,
        y: u64,
    }
    let mut ans = 0;
    if a > 2 * c && b > 2 * c {
        ans += x.max(y) * 2 * c;
    } else if a > 2 * c  {
        ans += x * 2 * c;
        ans += y.saturating_sub(x) * b;
    } else if b > 2 * c {
        ans += y * 2 * c;
        ans += x.saturating_sub(y) * a;
    } else if a + b > 2 * c {
        ans += 2 * c * x.min(y);
        if x < y {
            ans += (y - x) * b;
        } else {
            ans += (x - y) * a;
        }
    } else {
        ans += a * x + b * y;
    }

    println!("{}", ans);
}
