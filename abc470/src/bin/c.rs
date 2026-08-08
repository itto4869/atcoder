use std::collections::{HashMap, HashSet};

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut a = vec![0usize; n];
    let mut res = 0;
    let mut set = HashSet::new();
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: Usize1,
            }
            set.insert(x);

            res = res ^ a[x] ^ a[x] + 1;
            a[x] += 1;

            println!("{}", res);
        } else {
            let mut rm_v = Vec::new();
            for &idx in &set {
                res = res ^ a[idx] ^ (a[idx] - 1);
                if a[idx] == 1 {
                    rm_v.push(idx);
                }
                a[idx] -= 1;
            }

            for idx in rm_v {
                set.remove(&idx);
            }
            println!("{}", res);
        }
    }
}
