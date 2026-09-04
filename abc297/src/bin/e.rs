use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut pq = BinaryHeap::new();
    let mut set = HashSet::new();
    for &ai in &a {
        if set.contains(&ai) {
            continue;
        }
        pq.push(Reverse(ai));
        set.insert(ai);
    }

    let mut ans = 0;
    for _ in 0..k {
        ans = pq.pop().unwrap().0;
        for &ai in &a {
            let x = ans + ai;
            if set.contains(&x) {
                continue;
            }
            set.insert(x);
            pq.push(Reverse(x));
        }
    }

    println!("{}", ans);
}
