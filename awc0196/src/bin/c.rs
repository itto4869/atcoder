use ac_library::{Segtree, Max};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }
    let mut seg_a = Segtree::<Max<usize>>::new(n);
    let mut seg_b = Segtree::<Max<usize>>::new(n);
    for i in 0..n {
        seg_a.set(i, ab[i].0);
        seg_b.set(i, ab[i].1);
    }

    let mut ans = usize::MAX;
    for i in 0..n {
        let (a, b) = ab[i];
        let max_a = seg_a.prod(0..i).max(seg_a.prod((i + 1)..n));
        let max_b = seg_b.prod(0..i).max(seg_b.prod((i + 1)..n));

        let mut cnt = 0;
        if max_a >= a {
            cnt += max_a - a + 1;
        }

        if max_b >= b {
            cnt += max_b - b + 1;
        }

        ans = ans.min(cnt);
    }

    println!("{}", ans);
}
