use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    }
    let mut ans = 0;
    for i in 0..=a {
        for j in 0..=b {
            for k in 0..=c {
                if x == (500 * i + 100 * j + 50 * k) {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
