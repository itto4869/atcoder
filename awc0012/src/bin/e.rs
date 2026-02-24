use ac_library::Segtree;
use ac_library::segtree::Max;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    }
    let mut segtree = Segtree::<Max<i64>>::new(n);
    segtree.set(0, a[0]);
    for i in 1..n {
        let l = i.saturating_sub(k);
        let max_value = segtree.prod(l..i);
        segtree.set(i, max_value + a[i]);
    }

    let ans = segtree.get(n - 1);
    println!("{}", ans);
}
