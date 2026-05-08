use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        h: usize,
    }
    let mut ans = 0usize;
    let mut x = h;
    let mut cnt = 1;
    while x > 1 {
        ans += cnt;
        x /= 2;
        cnt *= 2;
    }

    if x == 1 {
        ans += cnt;
    }
    println!("{}", ans);
}