use proconio::{fastout, input};

#[fastout]
fn main() {
    let mut ans = 0;
    for n in 0..64 {
        input! {
            a: usize,
        }
        ans += a * 2usize.pow(n);
    }

    println!("{}", ans);
}
