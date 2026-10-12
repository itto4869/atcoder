use ac_library::{Segtree, Additive};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let mut segtree = Segtree::<Additive<usize>>::from(a);
    for _ in 0..q {
        input! {
            c: usize,
        }
        if c == 1 {
            input! {
                i: Usize1,
                x: usize,
            }
            segtree.set(i, x);
        } else if c == 2 {
            input! {
                l: Usize1,
                t: usize,
            }
            let ans = segtree.max_right(l, |&a| a < t);
            println!("{}", ans + 1);
        } else {
            input! {
                r: Usize1,
                t: usize,
            }
            let ans = segtree.min_left(r + 1, |&a| a < t);
            println!("{}", ans);
        }
    }
}