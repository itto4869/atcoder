use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut map = HashMap::new();
    for _ in 0..n {
        input! {
            s: String,
        }
        *map.entry(s).or_insert(0) += 1;
    }

    input! {
        m: usize,
    }

    for _ in 0..m {
        input! {
            t: String,
        }
        *map.entry(t).or_insert(0) -= 1;
    }

    let mut ans = 0;
    for (_, k) in map.iter() {
        ans = ans.max(*k);
    }

    println!("{}", ans);
}
