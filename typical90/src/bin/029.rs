use ac_library::{LazySegtree, MapMonoid, Max};
use cp_library::data_structure::lazy_segtree_map_monoid::RangeAssignMax;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        w: usize,
        n: usize,
    }
    let v = vec![Some(0); w + 1];
    let mut seg = LazySegtree::<RangeAssignMax>::from(v);

    for _ in 0..n {
        input! {
            l: Usize1,
            r: Usize1,
        }

        let h = seg.prod(l..=r).unwrap();
        seg.apply_range(l..=r, Some(h + 1));

        println!("{}", h + 1);
    }
}