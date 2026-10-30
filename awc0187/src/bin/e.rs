use ac_library::{Additive, LazySegtree, Segtree};
use cp_library::data_structure::lazy_segtree_map_monoid::RangeAddMax;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        v: [usize; n],
    }
    let mut lazy_seg = LazySegtree::<RangeAddMax<usize>>::new(n);
    for i in 0..n {
        lazy_seg.set(i, Some(v[i]));
    }
    let mut seg = Segtree::<Additive<usize>>::new(n);
    let mut seg2 = Segtree::<Additive<usize>>::new(n);
    for i in 0..n {
        seg2.set(i, 1);
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                l: Usize1,
                r: usize,
                x: usize,
            }
            lazy_seg.apply_range(l..r, x);
        } else if t == 2 {
            input! {
                p: Usize1,
            }
            let v = lazy_seg.get(p).unwrap();
            seg.set(p, v);
            seg2.set(p, 0);
        } else {
            input! {
                k: usize
            }
            let j = seg2.max_right(0, |&a| a < k);
            let res = seg.prod(0..j);
            println!("{}", res);
        }
    }
}
