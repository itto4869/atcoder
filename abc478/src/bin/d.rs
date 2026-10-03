use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut lrx: [(Usize1, Usize1, usize); q],
    }
    
    let mut v = vec![Vec::new(); q + 1];
    for (l, r, x) in lrx {
        v[x].push((l, r));
    }

    for lr in &mut v {
    lr.sort_unstable_by_key(|&(l, _)| l);
}

let mut imos = vec![0_i32; n + 1];

for lr in v {
    if lr.is_empty() {
        continue;
    }

    let (mut l_p, mut r_p) = lr[0];

    for &(l, r) in lr.iter().skip(1) {
        if r_p < l {
            imos[l_p] += 1;
            imos[r_p + 1] -= 1;

            l_p = l;
            r_p = r;
        } else {
            r_p = r_p.max(r);
        }
    }

    imos[l_p] += 1;
    imos[r_p + 1] -= 1;
}

    //println!("{:?}", imos);
    for i in 1..n {
        imos[i] += imos[i - 1];
    }

    imos.pop();
    println!("{}", imos.iter().format(" "));
}
