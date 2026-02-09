use ac_library::Segtree;
use ac_library::segtree::{Max, Min};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        h: [i64; n],
    }
    let mut max_segtree = Segtree::<Max<i64>>::new(n);
    let mut min_segtree = Segtree::<Min<i64>>::new(n);
    for i in 0..n {
        max_segtree.set(i, h[i]);
        min_segtree.set(i, h[i]);
    }

    let mut ans = 0;
    for i in 0..=(n - k) {
        let max_h = max_segtree.prod(i..(i + k));
        let min_h = min_segtree.prod(i..(i + k));
        ans = ans.max(max_h - min_h);
    }

    println!("{}", ans);
}
