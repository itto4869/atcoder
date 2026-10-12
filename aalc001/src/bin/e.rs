use ac_library::{Monoid, Segtree};
use proconio::{fastout, input, marker::{Chars, Usize1}};

const MOD: usize = 998244353;
#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        s: Chars,
    }
    let mut segtree = Segtree::<F>::new(n);
    for i in 0..n {
        let v = if s[i] == 'R' {
            (a[i], 0, 0, 0, 0, 0)
        } else if s[i] == 'G' {
            (0, a[i], 0, 0, 0, 0)
        } else {
            (0, 0, a[i], 0, 0, 0)
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
            let v = if s[i] == 'R' {
                (x, 0, 0, 0, 0, 0)
            } else if s[i] == 'G' {
                (0, x, 0, 0, 0, 0)
            } else {
                (0, 0, x, 0, 0, 0)
            };

            segtree.set(i, v);
        } else {
            input! {
                l: Usize1,
                r: Usize1
            }
            let ans = segtree.prod(l..=r);
            println!("{}", ans.5);
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
        ((a.0 + b.0) % MOD, (a.1 + b.1) % MOD, (a.2 + b.2) % MOD, (a.3 + b.3 + a.0 * b.1) % MOD, (a.4 + b.4 + a.1 * b.2) % MOD, (a.5 + b.5 + a.3 * b.2 + a.0 * b.4) % MOD)
    }
}