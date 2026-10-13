use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = vec![0; n + 1];
    for a in 1..=n.isqrt() {
        for b in a..=n {
            if a * b > n {
                break;
            }
            else {
                if a != b {
                    v[a * b] += 2;
                } else {
                    v[a * b] += 1;
                }
            }
        }
    }

    let mut ans = 0;
    for i in 1..n {
        ans += v[i] * v[n - i];
    }
    
    println!("{}", ans);
}
