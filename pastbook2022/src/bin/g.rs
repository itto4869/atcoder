use ac_library::Segtree;
use ac_library::Min;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let mut segtree = Segtree::<Min<usize>>::from_iter(a.into_iter());

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: usize,
                y: usize,
            }
            segtree.set(x, y);
        } else {
            input! {
                x: usize,
                y: usize,
            }
            let res = segtree.prod(x..y);
            println!("{}", res);
        }
    }
}
