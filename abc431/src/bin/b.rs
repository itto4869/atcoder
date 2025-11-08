use std::collections::HashMap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        mut x: u64,
        n: usize,
        w: [u64; n],
        q: usize,
    }
    let mut cnt = HashMap::new();
    for _ in 0..q {
        input! {
            p: Usize1,
        }
        *cnt.entry(p).or_insert(0) += 1;

        if cnt.get(&p).unwrap() % 2 == 1 {
            x += w[p];
        } else {
            x -= w[p];
        }

        println!("{}", x);
    }
}
