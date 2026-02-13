use ac_library::Segtree;
use ac_library::segtree::Max;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    }
    let mut segtree = Segtree::<Max<u64>>::new(n);
    for i in 0..n {
        segtree.set(i, a[i]);
    }

    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
        }
        let res = segtree.prod(l..=r);
        println!("{}", res);
    }
}
