use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            l: usize,
            x: [u64; l],
        }

        let mut res = 0;
        for xi in x {
            if !set.contains(&xi) {
                res = xi;
                set.insert(xi);
                break;
            }
        }

        println!("{}", res);
    }
}
