use std::collections::{BinaryHeap, HashSet};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        g: u64,
        mut f: u64,
        mut pr: [(u64, u64); n],
    }
    pr.sort_unstable();
    pr.push((g, 0));
    let mut prev = 0;
    let mut heap = BinaryHeap::new();
    let mut ans = 0;
    for i in 0..=n {
        let (p, r) = pr[i];
        while p - prev > f {
            if let Some(v) = heap.pop() {
                f += v;
                ans += 1;
            } else {
                println!("-1");
                return;
            }
        }
        heap.push(r);
        f -= p - prev;
        prev = p;
    }

    println!("{}", ans);
}
