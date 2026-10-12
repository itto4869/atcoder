use ac_library::{Monoid, Segtree};
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let mut segtree = Segtree::<F>::new(n);
    for i in 0..n {
        segtree.set(i, (a[i], 1, a[i], 1, 1, 1));
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
            segtree.set(i, (x, 1, x, 1, 1, 1));
        } else {
            input! {
                l: Usize1,
                r: Usize1
            }
            let ans = segtree.prod(l..=r);
            println!("{}", ans.4);
        }
    }
}

struct F;
impl Monoid for F {
    type S = (usize, usize, usize, usize, usize, usize);
    fn identity() -> Self::S {
        (0, 0, 0, 0, 0, 0)
    }

    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        if a.1 == 0 {
            *b
        } else if b.1 == 0 {
            *a
        } else {
            let v1 = if a.1 == a.5 {
                if a.0 == b.0 {
                    a.1 + b.1
                } else {
                    a.1
                }
            } else {
                a.1
            };

            let v3 = if b.3 == b.5 {
                if b.2 == a.2 {
                    a.3 + b.3
                } else {
                    b.3
                }
            } else {
                b.3
            };

            let tmp = if a.2 == b.0 {
                a.3 + b.1
            } else {
                0
            };

            let v4 = v1.max(v3).max(a.4).max(b.4).max(tmp);
            let v5 = a.5 + b.5;
            (a.0, v1, b.2, v3, v4, v5)
        }
    }
}