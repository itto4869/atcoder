use std::collections::BinaryHeap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
        v: i64,
    }
    let mut pq = BinaryHeap::new();
    for _ in 0..q {
        input! {
            f: usize,
        }

        if f == 1 {
            input! {
                t: i64,
                w: i64,
            }
            pq.push(w - t);
        } else {
            input! {
                t: i64,
            }
            if let Some(x) = pq.pop() {
                println!("{}", (x + t).min(v));
            } else {
                println!("-1");
            }
        }
    }
}
