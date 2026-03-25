use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        l: i64,
        r: i64,
    }
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        input! {
            t: i64,
        }
        if l <= t && t <= r {
            v.push(0);
        } else if t < l {
            v.push(l - t);
        } else {
            v.push(t - r);
        }
    }

    v.sort_unstable();
    let mut ans = 0;
    for i in 0..k {
        ans += v[i];
    }

    println!("{}", ans);
}
