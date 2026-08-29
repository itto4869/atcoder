use std::ops::Add;

use ac_library::{Max, Monoid, Segtree, Additive};
use proconio::{fastout, input, marker::{Chars, Usize1}};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
    }
    let mut v = vec![false; n];
    let mut segtree = Segtree::<Additive<i64>>::new(n);
    for i in 0..n {
        if s[i] == 'A' {
            segtree.set(i, 1);
        } else {
            segtree.set(i, -1);
        }
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                i: Usize1,
                c: char,
            }

            if c == 'A' {
                segtree.set(i, 1);
            } else {
                segtree.set(i, -1);
            }
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            }
            let b = segtree.prod(l..=r);
            if b < 0 {
                println!("No");
            } else {
                println!("Yes");
            }
        }
    }
}
