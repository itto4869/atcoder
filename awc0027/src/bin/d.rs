use std::collections::{BinaryHeap, HashMap};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut hs: [(u64, u64); n],
        mut p: [u64; m],
    }
    p.sort_unstable();

    hs.sort_unstable();
    let mut map = HashMap::new();
    for &(h, s) in &hs {
        *map.entry(h).or_insert(0) += 1;
    }

    let mut v: Vec<(u64, u64)> = map.into_iter().collect();
    v.sort_unstable();
    let mut heap = BinaryHeap::new();
    let mut idx = 0;
    let mut v_idx = 0;

    let mut ans = 0u64;
    for &pi in &p {
        while v_idx < v.len() && v[v_idx].0 <= pi {
            let (_, cnt) = v[v_idx];
            for _ in 0..cnt {
                heap.push(hs[idx].1);
                idx += 1;
            }

            v_idx += 1;
        }
        if let Some(s) = heap.pop() {
            ans += s;
        } else {
            println!("-1");
            return;
        }
    }

    println!("{}", ans);
}
