use std::collections::HashSet;

use cp_library::{utils::yes_no, yes_no};
use proconio::{fastout, input, marker::{Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
        m: usize,
    }
    let mut sets = vec![vec![HashSet::new(); 15]; 15];
    let mut s = Vec::new();
    for _ in 0..m {
        input! {
            si: Chars
        }
        let length = si.len();
        for i in 1..=length {
            sets[length][i].insert(si[i - 1]);
        }

        s.push(si);
    }

    for si in s {
        let mut ok = true;
        
        if si.len() != n {
            println!("No");
            continue;
        }

        for i in 0..n {
            let (a, b) = ab[i];
            if !sets[a][b].contains(&si[i]) {
                ok = false;
                break;
            }
        }

        yes_no!(ok);
    }
}
