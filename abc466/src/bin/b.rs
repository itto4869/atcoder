use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
    }
    let mut v = vec![-1; m];
    for _ in 0..n {
        input! {
            c: Usize1,
            s: i64,
        }
        v[c] = v[c].max(s);
    }

    println!("{}", v.iter().format(" "));
}
