use std::collections::HashSet;

use cp_library::utils::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut set = HashSet::new();
    for _ in 0..q {
        input! {
            t: u8,
            x: u64,
        }
        if t == 1 {
            set.insert(x);
        } else {
            yes_no(set.contains(&x));
        }
    }
}
