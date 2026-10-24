use std::collections::HashMap;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut cnt = 1;
    let mut map = HashMap::new();
    for _ in 0..q {
        input! {
            s: String,
        }
        if let Some(v) = map.get(&s) {
            println!("{}", v);
        } else {
            map.insert(s, cnt);
            println!("{}", cnt);
            cnt += 1;
        }
    }
}
