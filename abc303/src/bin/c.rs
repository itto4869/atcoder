use std::collections::HashSet;

use cp_library::yes_no;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        _: usize,
        m: usize,
        mut h: i64,
        k: i64,
        s: Chars
    }
    let mut set = HashSet::new();
    for _ in 0..m {
        input! {
            x: i64,
            y: i64,
        }
        set.insert((x, y));
    }

    let mut point = (0, 0);
    let mut ok = true;
    for c in s {
        match c {
            'R' => { point.0 += 1 },
            'L' => { point.0 -= 1 },
            'U' => { point.1 += 1 },
            'D' => { point.1 -= 1 },
            _ => unreachable!(),
        }
        
        h -= 1;
        if h < 0 {
            ok = false;
            break;
        }
        
        if h < k && set.contains(&point) {
            set.remove(&point);
            h = k;
        }
    }

    yes_no!(ok);
}
