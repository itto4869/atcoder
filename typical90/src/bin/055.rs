use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: u64,
        q: u64,
        a: [u64; n],
    }
    let mut ans = 0u64;
    for comb in a.into_iter().combinations(5) {
        let mut m = 1;
        for v in comb {
            m = (m * v) % p;
        }

        if m % p == q {
            ans += 1;
        }
    }

    println!("{}", ans);
}
