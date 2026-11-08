use std::collections::HashSet;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut v = vec![Vec::new(); m];
    for i in 0..m {
        input! {
            c: usize,
        }
        for _ in 0..c {
            input! {
                a: usize,
            }
            v[i].push(a);
        }
    }

    let mut ans = 0;
    for mask in 1..(1 << m) {
        let mut set = HashSet::new();
        for i in 0..m {
            if mask & (1 << i) != 0 {
                for &vi in &v[i] {
                    set.insert(vi);
                }
            }
        }

        if set.len() == n {
            ans += 1;
        }
    }

    println!("{}", ans);
}
