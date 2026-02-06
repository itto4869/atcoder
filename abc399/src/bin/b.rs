use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [u64; n],
    }
    let mut s = p.clone();
    s.sort();
    s.reverse();
    let mut r = 1;
    let mut map = HashMap::new();
    map.insert(s[0], 1);
    for i in 1..n {
        r += 1;
        if s[i] != s[i - 1] {
            map.insert(s[i], r);
        } 
    }

    for i in 0..n {
        println!("{}", map.get(&p[i]).unwrap());
    }
}
