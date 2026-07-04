use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = vec![0; n];
    let mut l = 0;
    let mut r = n - 1;
    let mut flag = 0;
    for idx in (0..n).rev() {
        let c = s[idx];
        if c == 'o' {
            if flag == 0 {
                ans[l] = idx + 1;
                flag = 1;
                l += 1;
            } else {
                ans[r] = idx + 1;
                flag = 0;
                r -= 1;
            }
        } else {
            if flag == 0 {
                ans[r] = idx + 1;
                r -= 1;
            } else {
                ans[l] = idx + 1;
                l += 1;
            }
        }
    }

    println!("{}", ans.iter().format(" "));
}
