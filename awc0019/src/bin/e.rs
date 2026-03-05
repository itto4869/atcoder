use std::collections::BinaryHeap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut wd: [(u64, u64); n],
    }
    
    wd.sort_by_key(|&(w, d)| w + d);

    let mut curr_w = 0;
    let mut heap = BinaryHeap::new();

    for (w, d) in wd {
        curr_w += w;
        heap.push(w);

        if curr_w - w > d {
            if let Some(max_w) = heap.pop() {
                curr_w -= max_w;
            }
        }
    }

    println!("{}", heap.len());
}
