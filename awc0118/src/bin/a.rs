use std::collections::HashSet;

use cp_library::yes_no;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [[i64; m]; n],
    }
    let mut sets = vec![HashSet::new(); m];
    for i in 0..n {
        for j in 0..m {
            if s[i][j] == -1 {
                continue;
            }

            sets[j].insert(s[i][j]);
        }
    }

    let mut ok = true;
    for set in sets {
        if set.len() >= 2 {
            ok = false;
            break;
        }
    }

    yes_no!(ok);
}
