use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [u64; n],
    }
    let mut map = HashMap::new();
    for &ai in &a {
        *map.entry(ai).or_insert(0) += ai;
    }

    let mut arr: Vec<(u64, u64)> = map.into_iter().collect();
    arr.sort_by(|a, b| a.1.cmp(&b.1));
    arr.reverse();
    let mut d = 0;
    for i in 0..k.min(arr.len()) {
        d += arr[i].1;
    }

    let ans = a.iter().sum::<u64>() - d;
    println!("{}", ans);
}
