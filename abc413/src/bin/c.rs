use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut a = Vec::with_capacity(q);
    let mut idx = 0;
    for _ in 0..q {
        input! {
            c: u64,
        }
        if c == 1 {
            input! {
                c: u64,
                x: u64,
            }
            a.push((c, x));
        } else {
            input! {
                mut k: u64,
            }
            let mut ans = 0;
            while k > 0 {
                let (c, x) = a[idx];
                if c <= k {
                    ans += x * c;
                    k -= c;
                    idx += 1;
                } else {
                    ans += x * k;
                    a[idx] = (c - k, x);
                    break;
                }
            }
            println!("{}", ans);
        }
    }
}
