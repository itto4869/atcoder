use std::collections::HashMap;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [i64; n],
    }
    let mut map = HashMap::new();
    for _ in 0..m {
        input! {
            u: usize,
            v: usize,
            b: i64,
        }
        map.insert((u, v), b);
    }

    let mut ans = i64::MIN;
    for comb in (1..=n).combinations(k) {
        let mut res = 0;
        for &i in &comb {
            res += a[i - 1];
        }

        for subcomb in comb.into_iter().combinations(2) {
            res -= map.get(&(subcomb[0], subcomb[1])).unwrap_or(&0);
        }

        ans = ans.max(res);
    }

    println!("{}", ans);
}
