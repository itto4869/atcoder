use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut v = vec![1; n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        v[a] += 1;
        v[b] += 1;
    }

    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        let k = n - v[i];
        let cnt = if k < 3 {
                0
            } else {
                (k * (k - 1) * (k - 2)) / 6
            };
        
        ans.push(cnt);
    }

    println!("{}", ans.iter().format(" "));
}
