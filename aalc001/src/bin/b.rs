use ac_library::LazySegtree;
use cp_library::data_structure::lazy_segtree_map_monoid::RangeAddMin;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let a: Vec<Option<i64>> = a.into_iter().map(|x| Some(x)).collect();
    let mut segtree = LazySegtree::<RangeAddMin>::from(a);
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                l: Usize1,
                r: Usize1,
                x: i64,
            }
            segtree.apply_range(l..=r, x);
        } else {
            input! {
                l: Usize1,
                r: Usize1,
            }
            let ans = segtree.prod(l..=r).unwrap();
            println!("{}", ans);
        }
    }
}
