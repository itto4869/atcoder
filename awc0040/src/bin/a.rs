use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: u64,
        p: [u64; n],
    }
    let mut ans = s;
    for _ in 0..m {
        input! {
            t: Usize1,
            q: u64,
        }
        let x = p[t] * q;
        ans += x - x / 2;
    }

    println!("{}", ans);
}
