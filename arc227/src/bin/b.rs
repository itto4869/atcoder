use std::{cmp::Reverse, collections::{BinaryHeap, HashSet}};

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut v = vec![0; n];
    for ai in a {
        v[ai] += 1;
    }

    let mut pq = BinaryHeap::new();
    pq.push(0);

    let mut ans = Vec::new();
    let mut ook = true;
    for i in 0..n {
        let mut ok = false;
        while let Some(top) = pq.pop() {
            if v[top] > 0 {
                ans.push(top);
                v[top] -= 1;
                ok = true;
                pq.push(top);
                pq.push(i + 1);
                break;
            }
        }

        if !ok {
            ook = false;
            break;
        }
    }

    if ook {
        println!("Yes\n{}", ans.iter().format(" "));
    } else {
        println!("No");
    }
}
