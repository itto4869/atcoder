use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut map: HashMap<usize, u64> = HashMap::new();
    for &ai in &a {
        if let Some(k) = map.get(&(ai - 1)) {
            map.insert(ai, k + 1);
        } else {
            map.insert(ai, 1);
        }
    }

    println!("{}", map.values().max().unwrap());
}
