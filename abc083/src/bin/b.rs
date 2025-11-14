use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }
    let mut ans = 0;
    for i in 1..=n {
        let mut m = i;
        let mut num = 0;
        while m > 0 {
            num += m % 10;
            m /= 10;
        }
        if num >= a && num <= b {
            ans += i;
        }
    }
    println!("{}", ans);
}
