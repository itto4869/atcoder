use std::collections::HashSet;

use cp_library::utils::yes_no;
use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    let mut v = Vec::new();
    v.push(Vec::new());
    let mut set = HashSet::new();
    let mut last_idx = 0;
    let mut ok = true;
    for &c in &s {
        if c != b'(' && c != b')' {
            if set.contains(&c) {
                ok = false;
                break;
            } else {
                set.insert(c);
                v[last_idx].push(c);
            }
        } else if c == b'(' {
            last_idx += 1;
            v.push(Vec::new());
        } else {
            for t in &v[last_idx] {
                set.remove(t);
            }
            v.remove(last_idx);
            last_idx -= 1;
        }
    }

    yes_no(ok);
}
