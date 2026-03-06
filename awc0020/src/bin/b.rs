use std::collections::HashMap;

use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: i64,
        d: [i64; n],
    }
    let mut map = HashMap::new();
    for _ in 0..m {
        input! {
            p: Usize1,
            r: i64,
        }
        map.insert(p, r);
    }

    let mut bate = false;
    for i in 0..n {
        if bate {
            s -= 2 * d[i];
        } else {
            s -= d[i];
        }

        if s <= 0 {
            bate = true;
        }

        if let Some(r) = map.get(&i) {
            s += r;
        }
    }

    println!("{}", s);
}
