use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: u64,
        a: [u64; n],
    }
    let left_a = a[..(n / 2)].to_vec();
    let right_a = a[(n / 2)..].to_vec();

    let mut left_map: HashMap<u64, u64> = HashMap::new();
    for mask in 0..(1 << left_a.len()) {
        let mut total = 0;
        for i in 0..left_a.len() {
            if mask & (1 << i) != 0 {
                total += left_a[i];
            }
        }
        *left_map.entry(total).or_insert(0) += 1;
    }

    let mut right_map: HashMap<u64, u64> = HashMap::new();
    for mask in 0..(1 << right_a.len()) {
        let mut total = 0;
        for i in 0..right_a.len() {
            if mask & (1 << i) != 0 {
                total += right_a[i];
            }
        }
        *right_map.entry(total).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (key, value) in left_map {
        if key > s {
            continue;
        }

        let target = s - key;
        if let Some(&cnt) = right_map.get(&target) {
            ans += value * cnt;
        }
    }

    println!("{}", ans);
}
