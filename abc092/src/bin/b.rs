use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: u64,
        x: u64,
        a: [u64; n],
    }
    let mut ans = x;
    for i in 0..n {
        let ai = a[i];
        let mut curr = 1;
        while curr <= d {
            ans += 1;
            curr += ai;
        }
    }
    println!("{}", ans);
}
