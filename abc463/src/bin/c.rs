use std::collections::HashMap;

use ac_library::{Max, Segtree};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        hl: [(usize, usize); n],
        q: usize,
    }
    let mut map = HashMap::new();
    let mut idx = 0usize;
    for &(h, l) in &hl {
        if map.contains_key(&l) {
            continue;
        } else {
            map.insert(l, idx);
            idx += 1;
        }
    }

    let mut map2: HashMap<usize, usize> = HashMap::new();
    for &(h, l) in &hl {
        let &idx = map.get(&l).unwrap();
        if let Some(&value) = map2.get(&idx) {
            map2.insert(idx, value.max(h));
        } else {
            map2.insert(idx, h);
        }
    }
    let mut segtree = Segtree::<Max<usize>>::new(map2.len());
    for (idx, h) in map2 {
        segtree.set(idx, h);
    }


    let mut lv = Vec::new();
    for (h, l) in hl {
        if lv.is_empty() {
            lv.push(l);
        } else {
            let &pre = lv.last().unwrap();
            if pre == l {
                continue;
            } else {
                lv.push(l);
            }
        }
    }
    for _ in 0..q {
        input! {
            t: usize,
        }

        let li = lv.partition_point(|&x| x < (t + 1));
        let ans = segtree.prod(li..);
        println!("{}", ans);
    }
}
