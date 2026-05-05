use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ab = Vec::with_capacity(n);
    for i in 0..n {
        input! {
            a: u64,
            b: u64,
        }
        ab.push((i + 1, a, b));
    }
    ab.sort_by(|a, b| {
        let l = a.1 * (b.1 + b.2);
        let r = b.1 * (a.1 + a.2);
        if l.cmp(&r) == std::cmp::Ordering::Equal {
            a.0.cmp(&b.0)
        } else {
            r.cmp(&l)
        }
    });

    let ans = ab.iter().map(|(i, _, _)| i);
    println!("{}", ans.format(" "));
}
