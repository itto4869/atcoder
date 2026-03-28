use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut heap = BinaryHeap::new();
    for _ in 0..q {
        input! {
            t: usize,
            h: u64,
        }
        if t == 1 {
            heap.push(Reverse(h));
        } else {
            while let Some(hi) = heap.pop() {
                let hi = hi.0;
                if hi > h {
                    heap.push(Reverse(hi));
                    break;
                }
            }
        }

        println!("{}", heap.len());
    }
}
