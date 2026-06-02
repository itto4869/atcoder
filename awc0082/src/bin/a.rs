use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }
    let mut ans = 0usize;
    for _ in 0..m {
        input! {
            s: usize,
            p: Usize1,
            d: usize,
        }
        if s == 0 {
            ans += a[p] * d;
        } else {
            ans += a[p].saturating_sub(k) * d;
        }
    }

    println!("{}", ans);
}
