use ac_library::FenwickTree;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }
    let mut bit = FenwickTree::new(n, 0);
    for i in 0..n {
        bit.add(i, a[i]);
    }

    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 0 {
            input! {
                p: usize,
                x: usize,
            }
            bit.add(p, x);
        } else {
            input! {
                l: usize,
                r: usize,
            }
            let res = bit.sum(l..r);
            println!("{}", res);
        }
    }
}
