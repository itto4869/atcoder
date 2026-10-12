use ac_library::{Segtree, Min};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let mut segtree = Segtree::<Min<usize>>::from(a);
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                i: Usize1,
                x: usize,
            }
            segtree.set(i, x);
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            }
            let ans = segtree.prod(l..=r);
            println!("{}", ans);
        }
    }
}
