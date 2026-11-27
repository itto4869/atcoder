use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: Usize1,
        q: Usize1,
        r: Usize1,
        s: Usize1,
        a: [usize; n],
    }
    let mut b = Vec::with_capacity(n);
    for i in 0..p {
        b.push(a[i]);
    }

    for i in r..=s {
        b.push(a[i]);
    }

    for i in (q + 1)..r {
        b.push(a[i]);
    }

    for i in p..=q {
        b.push(a[i]);
    }

    for i in (s + 1)..n {
        b.push(a[i]);
    }

    println!("{}", b.iter().format(" "));
}
