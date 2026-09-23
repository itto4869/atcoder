use ac_library::{BitwiseXor, Segtree};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let mut segtree = Segtree::<BitwiseXor<usize>>::from_iter(a.into_iter());
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: Usize1,
                y: usize,
            }
            let p = segtree.get(x);
            segtree.set(x, p ^ y);
        } else {
            input! {
                x: Usize1,
                y: Usize1
            }
            let res = segtree.prod(x..=y);
            println!("{}", res);
        }
    }
}
