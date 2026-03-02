use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        lr: [(usize, usize); n],
    }
    let mut ans = 0.0;

    for i in 0..n {
        for j in (i + 1)..n {
            let (li, ri) = lr[i];
            let (lj, rj) = lr[j];

            let mut cnt = 0;
            let total = (ri - li + 1) * (rj - lj + 1);

            for x in li..=ri {
                for y in lj..=rj {
                    if x > y {
                        cnt += 1;
                    }
                }
            }

            ans += cnt as f64 / total as f64;
        }
    }

    println!("{}", ans);
}
