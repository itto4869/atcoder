use ac_library::Segtree;
use ac_library::Additive;
use proconio::marker::Usize1;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut segtree = Segtree::<Additive<usize>>::new(n);
    for i in 0..n {
        segtree.set(i, 0);
    }

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                p: Usize1,
                x: usize,
            }
            segtree.set(p, x);
        } else {
            input! {
                l: Usize1,
                r: Usize1
            }
            let res = segtree.prod(l..r);
            println!("{}", res);
        }
    }
}
