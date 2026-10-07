use std::collections::HashSet;

use ac_library::Dsu;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    
    let mut dsu = Dsu::new(2 * n);
    for i in 0..n {
        dsu.merge(2 * i, 2 * i + 1);
    }
    let mut set = HashSet::new();
    for _ in 0..m {
        input! {
            a: Usize1,
            b: char,
            c: Usize1,
            d: char,
        }
        
        let idx_a = if b == 'R' {
            2 * a
        } else {
            2 * a + 1
        };

        let idx_c = if d == 'R' {
            2 * c
        } else {
            2 * c + 1
        };

        if dsu.same(idx_a, idx_c) {
            set.insert(dsu.leader(idx_a));
        } else {
            dsu.merge(idx_a, idx_c);
        }
    }

    let x = set.len();
    let y = dsu.groups().len() - x;

    println!("{} {}", x, y);
}
