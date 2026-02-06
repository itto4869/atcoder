use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut s = Vec::with_capacity(2 * n);
    for _ in 0..n {
        input! {
            a: u64,
            b: u64,
        }
        s.push(b);
        s.push(a - b);
    }
    s.sort_unstable();
    s.reverse();
    
    let mut ans = 0;
    for i in 0..k {
        ans += s[i];
    }

    println!("{}", ans);
}
