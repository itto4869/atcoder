use std::u64;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            w: usize,
            c: [u64; n],
        }
        let mut costs = vec![0; 2 * w];
        for i in 0..n {
            costs[i % (2 * w)] += c[i];
        }

        let mut cost: u64 = costs[..w].iter().sum();
        let mut ans = cost;
        for i in 0..(2 * w) {
            let l = i;
            let r = (i + w) % (2 * w);
            cost = cost + costs[r] - costs[l];
            ans = ans.min(cost);
        }

        println!("{}", ans);
    }
}
