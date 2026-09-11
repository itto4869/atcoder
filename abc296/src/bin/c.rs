use std::collections::HashSet;

use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: i64,
    }
    let mut set = HashSet::new();
    for _ in 0..n {
        input! {
            a: i64,
        }
        set.insert(a);
    }

    let mut ok = false;
    for &ai in &set {
        if set.contains(&(ai - x)) {
            ok = true;
            break;
        }
    }

    yes_no!(ok);
}
