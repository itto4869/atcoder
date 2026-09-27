use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut pq1 = BinaryHeap::new();
    let mut pq2 = BinaryHeap::new();
    let mut set = HashSet::new();

    for i in 1..=n {
        pq1.push(Reverse(i));
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            let k = pq1.pop().unwrap();
            pq2.push(k);
        } else if t == 2 {
            input! {
                x: usize,
            }
            set.insert(x);
        } else {
            while let Some(k) = pq2.pop() {
                let k = k.0;
                if !set.contains(&k) {
                    println!("{}", k);
                    pq2.push(Reverse(k));
                    break;
                }
            }
        }
    }
}
