use std::collections::{HashMap, HashSet};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        _: usize,
        _: usize,
        q: usize,
    }
    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for _ in 0..q {
        input! {
            c: u8,
        }

        if c == 1 {
            input! {
                x: usize,
                y: usize,
            }
            map.entry(x).or_insert(HashSet::new()).insert(y);
        } else if c == 2 {
            input! {
                x: usize,
            }
            map.entry(x).or_insert(HashSet::new()).insert(0);
        } else {
            input! {
                x: usize,
                y: usize,
            }
            if let Some(set) = map.get(&x) {
                if set.contains(&0) || set.contains(&y) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            } else {
                println!("No");
            }
        }
    }
}
