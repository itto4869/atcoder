use std::collections::{HashMap, HashSet};

use cp_library::yes_no;
use proconio::{fastout, input, marker::{Bytes, Chars}};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Bytes,
        t: Bytes,
    }
    let mut ok = true;
    for i in 0..n {
        if t[i] == b'*' {
            continue;
        } else if t[i] != s[i] {
            ok = false;
            break;
        }
    }

    yes_no!(ok);
}
