use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = 0;
    for b in (k + 1)..=n {
        let base = b.saturating_sub(k);
        let m = (n - k + 1) / b;
        ans += m * base;
        ans += (n - (k + m * b - 1)).min(base);
        if k == 0 {
            ans -= 1;
        }
    }

    println!("{}", ans);
}
