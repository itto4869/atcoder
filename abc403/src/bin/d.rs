use std::collections::HashMap;
use proconio::{input, fastout};
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    }

    if d == 0 {
        let mut uniq = a.clone();
        uniq.sort_unstable();
        uniq.dedup();
        println!("{}", n - uniq.len());
        return;
    }

    let mut counts = HashMap::new();
    for &x in &a {
        *counts.entry(x).or_insert(0) += 1;
    }

    let mut visited = HashMap::new();
    let mut max_keep = 0;

    let mut keys: Vec<usize> = counts.keys().cloned().collect();
    keys.sort_unstable();

    for &start_val in &keys {
        if visited.contains_key(&start_val) {
            continue;
        }

        let mut chain = Vec::new();
        let mut curr = start_val;

        while let Some(&cnt) = counts.get(&curr) {
            visited.insert(curr, true);
            chain.push(cnt);
            curr += d;
        }

        let mut dp0 = 0;
        let mut dp1 = 0;

        for &cnt in &chain {
            let next_dp0 = max(dp0, dp1);
            let next_dp1 = dp0 + cnt;
            dp0 = next_dp0;
            dp1 = next_dp1;
        }

        max_keep += max(dp0, dp1);
    }
    
    println!("{}", n - max_keep);
}