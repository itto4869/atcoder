use std::u64;

use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Bytes,
        t: Bytes,
    }
    let mut ans = u64::MAX;
    for i in 0..(n - m + 1) {
        let mut cnt = 0;
        for j in 0..m {
            if s[i + j] < t[j] {
                cnt += 10 - (t[j] - s[i + j]) as u64;
            } else {
                cnt += (s[i + j] - t[j]) as u64;
            }
        }
        ans = ans.min(cnt);
    }

    println!("{}", ans);
}
