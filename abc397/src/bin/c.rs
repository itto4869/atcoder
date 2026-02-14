use std::collections::{HashMap, HashSet};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut l_set = HashSet::new();
    let mut r_map = HashMap::new();
    for &ai in &a {
        *r_map.entry(ai).or_insert(0) += 1;
    }

    let mut ans = 0;
    for i in 0..n {
        l_set.insert(a[i]);
        let &r_cnt = r_map.get(&a[i]).unwrap();
        if r_cnt > 1 {
            r_map.insert(a[i], r_cnt - 1);
        } else {
            r_map.remove(&a[i]);
        }

        ans = ans.max(l_set.len() + r_map.len());
    }

    println!("{}", ans);
}
