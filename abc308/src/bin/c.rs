use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v = Vec::with_capacity(n);
    for i in 1..=n {
        input! {
            a: u64,
            b: u64,
        }
        v.push((a, (a + b), i));
    }

    v.sort_by(|a, b| {
        let l = a.0 * b.1;
        let r = b.0 * a.1;
        l.cmp(&r).reverse()
    });

    let ans = v.iter().map(|&(_, _, idx)| idx);

    println!("{}", ans.format(" "));
}
