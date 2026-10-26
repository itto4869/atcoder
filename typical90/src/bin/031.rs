use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        w: [usize; n],
        b: [usize; n],
    }
    let mut dp = vec![vec![0usize; 2000]; 100];
    for wi in 0..=50 {
        for bi in 0..=1500 {
            let mut set = HashSet::new();

            if wi >= 1 {
                set.insert(dp[wi - 1][bi + wi]);
            }

            if bi >= 2 {
                for i in 1..=(bi / 2) {
                set.insert(dp[wi][bi - i]);
            }
            }

            let mut v: Vec<usize> = set.into_iter().collect();
            v.sort_unstable();
            for i in 0..=v.len() {
                if i == v.len() {
                    dp[wi][bi] = i;
                    break;
                }
                if v[i] != i {
                    dp[wi][bi] = i;
                    break;
                }
            }
        }
    }

    let mut ans = 0;
    for (&wi, &bi) in w.iter().zip(&b) {
        ans ^= dp[wi][bi];
    }

    if ans == 0 {
        println!("Second");
    } else {
        println!("First");
    }
}
