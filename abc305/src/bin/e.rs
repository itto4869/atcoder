use std::{cmp::Reverse, collections::{BinaryHeap, HashMap, HashSet}};
use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut graph = vec![Vec::new(); n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        graph[a].push(b);
        graph[b].push(a);
    }

    let mut map = HashMap::new();
    let mut pq = BinaryHeap::new();
    for _ in 0..k {
        input! {
            p: Usize1,
            h: usize,
        }
        pq.push((h, p))
    }

    let mut set = HashSet::new();
    
    while let Some((hi, pi)) = pq.pop() {
        set.insert(pi + 1);
        let next = &graph[pi];
        for &np in next {
            if let Some(&r) = map.get(&np) {
                if r < hi {
                    map.insert(np, hi);
                    if hi > 0 {
                        pq.push((hi - 1, np));
                    }
                } else {
                    continue;
                }
            } else {
                set.insert(pi + 1);
                map.insert(np, hi);
                if hi > 0 {
                    pq.push((hi - 1, np));
                }
            }
        }
    }

    let mut v: Vec<usize> = set.into_iter().collect();
    v.sort_unstable();
    println!("{}\n{}", v.len(), v.iter().format(" "));
}
