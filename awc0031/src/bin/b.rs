use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: usize,
    }
    let mut ans = 0;
    for _ in 0..t {
        input! {
            mut s: [u64; n],
        }
        s.sort_unstable();
        s.reverse();
        if s[0] >= 2 * s[1] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
