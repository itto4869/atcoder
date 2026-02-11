use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    let mut map = HashMap::new();
    for _ in 0..7 {
        input! {
            a: u64,
        }
        *map.entry(a).or_insert(0) += 1u64;
    }

    let mut counter: Vec<u64> = map.into_values().collect();
    counter.sort_unstable();
    counter.reverse();
    if counter.len() < 2 {
        println!("No");
    } else if counter[0] >= 3 && counter[1] >= 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
