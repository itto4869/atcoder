use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut ts = Vec::with_capacity(n);
    for i in 1..=n {
        input! {
            t: u64,
        }
        ts.push((t, i));
    }

    ts.sort();
    let ans = [ts[0].1, ts[1].1, ts[2].1];
    println!("{}", ans.iter().format(" "));
}
