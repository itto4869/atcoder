use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        mut a: usize,
        mut b: usize,
    }
    let mut ans = 0;
    while a != b {
        (a, b) = (a.max(b), a.min(b));
        if (a % b) == 0 {
            ans += a / b - 1;
            a = b;
        } else {
            ans += a / b;
            a = a % b;
        }
    }

    println!("{}", ans);
}
