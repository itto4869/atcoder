use std::collections::{HashMap, HashSet};

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: u64,
        mut ab: [(u64, u64); n],
    }
    let mut set = HashSet::new();
    let mut b_sum = 0;
    for &(a, b) in &ab {
        set.insert(a);
        b_sum += b;
    }

    let mut v: Vec<u64> = set.into_iter().collect();
    v.sort_unstable();

    let mut map = HashMap::new();
    let mut back_map = HashMap::new();
    for i in 0..v.len() {
        let a = v[i];
        map.insert(a, i);
        back_map.insert(i, a);
    }

    let mut imos = vec![0i64; n + 1];
    for &(a, b) in &ab {
        let &idx = map.get(&a).unwrap();
        imos[0] += b as i64;
        imos[idx] -= b as i64;
    }

    for i in 1..=n {
        imos[i] += imos[i - 1];
    }

    let idx = imos.partition_point(|&x| x > (k as i64));
    
    let mut ans = *back_map.get(&idx).unwrap();

    if b_sum > k {
        ans += 1;
    } else {
        ans = 1;
    }
    println!("{}", ans);
}
