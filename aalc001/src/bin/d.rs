use ac_library::{Monoid, Segtree};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
    }
    let mut segtree = Segtree::<F>::new(n);
    for i in 0..n {
        let v = if a[i] == 0 {
            (1, 0, 0, 0)
        } else if a[i] == 1 {
            (0, 1, 0, 0)
        } else {
            (0, 0, 1, 0)
        };

        segtree.set(i, v);
    }

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                i: Usize1,
                x: usize,
            }
            let v = if x == 0 {
                (1, 0, 0, 0)
            } else if x == 1 {
                (0, 1, 0, 0)
            } else {
                (0, 0, 1, 0)
            };

            segtree.set(i, v);
        } else {
            input! {
                l: Usize1,
                r: Usize1
            }

            let ans = segtree.prod(l..=r);
            println!("{}", ans.3);
        }
    }
}

struct F;
impl Monoid for F {
    type S = (usize, usize, usize, usize);
    fn identity() -> Self::S {
        (0, 0, 0, 0)
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 + b.0, a.1 + b.1, a.2 + b.2, a.3 + b.3 + a.1 * b.0 + a.2 * b.0 + a.2 * b.1)
    }
}