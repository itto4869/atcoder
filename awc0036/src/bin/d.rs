use std::collections::{HashMap, HashSet};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        se: [(u64, u64); n],
    }
    let mut v = Vec::with_capacity(2 * n);
    for &(s, e) in &se {
        v.push(s);
        v.push(e);
    }

    v.sort_unstable();
    let mut map = HashMap::new();
    for i in 0..v.len() {
        map.insert(v[i], i);
    }

    let mut imos = vec![0; n * 2 + 1];
    for &(s, e) in &se {
        let &s_idx = map.get(&s).unwrap();
        let &e_idx = map.get(&e).unwrap();

        imos[s_idx] += 1;
        imos[e_idx] -= 1;
    }

    for i in 1..imos.len() {
        imos[i] += imos[i - 1];
    }

    let ans = imos.iter().max().unwrap();
    println!("{}", ans);
}
