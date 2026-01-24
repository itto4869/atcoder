use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [u64; n],
    }
    let mut s = vec![0; n];
    s[0] = a[0];
    for i in 1..n {
        s[i] = s[i - 1] + a[i];
    }

    for _ in 0..q {
        input! {
            c: u64,
        }
        if c == 1 {
            input! {
                x: Usize1,
            }
            let l = a[x];
            let r = a[x + 1];
            if l < r {
                s[x] += r - l;
            } else {
                s[x] -= l - r;
            }

            a.swap(x, x + 1);
        } else {
            input! {
                l: Usize1,
                r: Usize1
            }
            let ans = if l > 0 {
                s[r] - s[l - 1]
            } else {
                s[r]
            };
            println!("{}", ans);
        }
    }
}
