use std::collections::HashSet;

use cp_library::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _: usize,
        s: Chars,
    }
    let mut p = (0i64, 0i64);
    let mut set = HashSet::new();
    set.insert(p);
    let mut ok = false;
    for c in s {
        match c {
            'R' => p = (p.0 + 1, p.1),
            'L' => p = (p.0 - 1, p.1),
            'U' => p = (p.0, p.1 + 1),
            'D' => p = (p.0, p.1 - 1),
            _ => unreachable!()
        }

        if !set.insert(p) {
            ok = true;
        }
    }

    yes_no!(ok);
}
