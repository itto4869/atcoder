use std::collections::HashSet;

use cp_library::yes_no;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut set = HashSet::new();
    let mut v = vec![false; m];
    let mut ok1 = true;
    for _ in 0..n {
        input! {
            f: Usize1,
        }
        if set.contains(&f) {
            ok1 = false;
        }

        set.insert(f);
        v[f] = true;
    }

    let mut ok2 = true;
    for vi in v {
        if !vi {
            ok2 = false;
            break;
        }
    }

    yes_no!(ok1);
    yes_no!(ok2);
}
