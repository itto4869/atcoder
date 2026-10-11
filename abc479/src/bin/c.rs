use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: usize,
        ab: [(usize, usize); n],
    }
    let mut ans = 0;
    for i in 1..=n {
        for comb in ab.iter().combinations(i) {
            let mut sum_a = 0;
            let mut sum_w = 0;
            for &(a, b) in comb {
                sum_a += a;
                sum_w += b;
            }

            if sum_w <= w {
                ans = ans.max(sum_a);
            }
        }
    }

    println!("{}", ans);
}
