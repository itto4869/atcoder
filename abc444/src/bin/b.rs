use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = 0;
    for i in 1..=n {
        let mut x = i;
        let mut d_sum = 0;
        while x > 0 {
            d_sum += x % 10;
            x /= 10;
        }

        if d_sum == k {
            ans += 1;
        }
    }

    println!("{}", ans);
}
