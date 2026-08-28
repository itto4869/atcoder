use std::collections::HashSet;

use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut v2 = vec![Vec::new(); n];
    let mut v3 = vec![HashSet::new(); 200_000 + 1];
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                i: usize,
                j: Usize1,
            }
            v2[j].push(i);
            v3[i].insert(j + 1);
        } else if t == 2 {
            input! {
                i: Usize1,
            }
            v2[i].sort_unstable();
            println!("{}", v2[i].iter().format(" "));
        } else {
            input! {
                i: usize,
            }
            let mut v: Vec<&usize> = v3[i].iter().collect();
            v.sort_unstable();
            println!("{}", v.iter().format(" "));
        }
    }
}
