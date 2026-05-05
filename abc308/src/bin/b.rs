use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String; n],
        d: [String; m],
        p0: u64,
    }
    let mut map = HashMap::new();
    for i in 0..m {
        input! {
            p: u64,
        }
        map.insert(&d[i], p);
    }
    let mut ans = 0;
    for ci in c {
        if let Some(&pi) = map.get(&ci) {
            ans += pi;
        } else {
            ans += p0;
        }
    }

    println!("{}", ans);
}
