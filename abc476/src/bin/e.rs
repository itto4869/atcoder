use std::collections::HashMap;

use ac_library::Segtree;
use ac_library::{Max, Min};
use itertools::Itertools;
use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        p: [usize; n],
        lr : [(Usize1, Usize1); m],
    }
    let mut max_segtree = Segtree::<Max<usize>>::new(n);
    let mut min_segtree = Segtree::<Min<usize>>::new(n);

    let mut map = HashMap::new();
    for i in 0..n {
        max_segtree.set(i, p[i]);
        min_segtree.set(i, p[i]);

        map.insert(p[i], i);
    }

    for (l, r) in lr {
        let max_v = max_segtree.prod(l..=r);
        let min_v = min_segtree.prod(l..=r);

        let max_idx = *map.get(&max_v).unwrap();
        let min_idx = *map.get(&min_v).unwrap();

        map.insert(max_v, min_idx);
        map.insert(min_v, max_idx);

        max_segtree.set(max_idx, min_v);
        max_segtree.set(min_idx, max_v);

        min_segtree.set(max_idx, min_v);
        min_segtree.set(min_idx, max_v);
    }

    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        let v = max_segtree.get(i);
        ans.push(v);
    }

    println!("{}", ans.iter().format(" "));
}
