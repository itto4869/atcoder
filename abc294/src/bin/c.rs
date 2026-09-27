use std::collections::HashMap;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut c = Vec::with_capacity(n + m);
    for i in 0..n {
        c.push(a[i]);
    }

    for i in 0..m {
        c.push(b[i]);
    }

    c.sort_unstable();
    let mut map = HashMap::new();
    for i in 0..(n + m) {
        map.insert(c[i], i + 1);
    }

    let mut ans = Vec::with_capacity(n);
    for i in 0..n {
        let &k = map.get(&a[i]).unwrap();
        ans.push(k);
    }

    println!("{}", ans.iter().format(" "));

    ans.clear();
    for i in 0..m {
        let &k = map.get(&b[i]).unwrap();
        ans.push(k);
    }

    println!("{}", ans.iter().format(" "));
}
