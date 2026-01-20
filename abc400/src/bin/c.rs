use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: u64,
    }
    let mut a = 2;
    let mut ans = 0;
    while a <= n {
        let k = n / a;
        let m = k.isqrt();
        let diff = m / 2;
        ans += m - diff;
        a *= 2;
    }

    println!("{}", ans);
}
