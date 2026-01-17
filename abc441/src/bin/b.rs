use std::{collections::HashSet, iter::FromIterator};

use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: Bytes,
        t: Bytes,
        q: usize,
    }
    let s_set: HashSet<u8> = HashSet::from_iter(s);
    let t_set: HashSet<u8> = HashSet::from_iter(t);

    for _ in 0..q {
        input! {
            w: Bytes
        }
        let mut takahasi = true;
        let mut aoki = true;
        for c in &w {
            if !s_set.contains(c) {
                takahasi = false;
            }

            if !t_set.contains(c) {
                aoki = false;
            }
        }

        if takahasi && aoki {
            println!("Unknown");
        } else if takahasi && !aoki {
            println!("Takahashi");
        } else {
            println!("Aoki");
        }
    }
}
