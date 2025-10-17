use std::collections::HashMap;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut count = HashMap::new();
    for ai in a {
        *count.entry(ai).or_insert(0u64) += 1;
    }
    let mut count: Vec<(&u64, &u64)> = count.iter().filter(|(_, b)| **b >= 2).collect();
    count.sort_by(|a, b| a.0.cmp(&b.0));
    count.reverse();
    
    let mut side = Vec::new();
    for (a, b) in count {
        if *b <= 1 {
            continue;
        } else if *b <= 3 {
            side.push(a);
        } else {
            side.push(a);
            if side.len() < 2 {
                side.push(a);
            }
        }
        if side.len() >= 2 {
            break;
        }
    }
    if side.len() < 2 {
        println!("0");
    } else {
        println!("{}", side[0] * side[1]);
    }
}
