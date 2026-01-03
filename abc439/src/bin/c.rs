use std::collections::HashMap;
use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut d = 1;
    let mut sqrt_ns = Vec::with_capacity(n);
    while d * d <= n {
        sqrt_ns.push(d * d);
        d += 1;
    }

    let mut ans = Vec::new();
    let mut map = HashMap::new();
    for i in 0..(sqrt_ns.len() - 1) {
        for j in (i + 1)..sqrt_ns.len() {
            let m = sqrt_ns[i] + sqrt_ns[j];
            if m > n {
                break;
            }
            *map.entry(m).or_insert(0) += 1;
        }
    }

    for (m, k) in map {
        if k == 1 {
            ans.push(m);
        }
    }

    ans.sort();
    println!("{}", ans.len());
    println!("{}", ans.iter().format(" "));
}
