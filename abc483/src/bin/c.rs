use ac_library::{Monoid, Segtree};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }
    let mut v = vec![0; n];
    let mut vv = vec![Vec::new(); n];
    for &(a, b) in &ab {
        v[a] += 1;
        vv[a].push(b);
    }

    let mut segtree = Segtree::<MaxPair>::new(n);
    for i in 0..n {
        segtree.set(i, (v[i], i as i64 * -1));
    }

    for k in 0..n {
        for &idx in &vv[k] {
            let u = segtree.get(k);
            segtree.set(k, (u.0 - 1, u.1));

            let x = segtree.get(idx);
            segtree.set(idx, (x.0 + 1, x.1));
        }

        let ans = segtree.prod(..n);
        println!("{}", ans.1 * -1 + 1);
        for &idx in &vv[k] {
            let u = segtree.get(k);
            segtree.set(k, (u.0 + 1, u.1));

            let x = segtree.get(idx);
            segtree.set(idx, (x.0 - 1, x.1));
        }
    }
}

struct MaxPair;

impl Monoid for MaxPair {
    type S = (usize, i64);

    fn identity() -> Self::S {
        (usize::MIN, i64::MIN)
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (*a).max(*b)
    }
}