use std::{cmp::Reverse, collections::BinaryHeap};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); m],
    }
    let mut start_at = vec![vec![]; n + 1];
    for (l, r) in lr {
        start_at[l].push(r);
    }

    let mut heap = BinaryHeap::new();

    let mut cnt = 0;

    for i in 1..=n {
        for &r in &start_at[i] {
            heap.push(Reverse(r));
        }

        if let Some(Reverse(r)) = heap.pop() {
            if r < i {
                println!("No");
                return;
            }
            cnt += 1;
        }
    }

    if cnt == m {
        println!("Yes");
    } else {
        println!("No");
    }
}
