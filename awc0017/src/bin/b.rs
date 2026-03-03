use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [u64; n],
    }
    for _ in 0..m {
        input! {
            b: Usize1
        }
        if b > 0 {
            a[b - 1] += 1;
        }

        a[b] += 1;

        if b < n - 1 {
            a[b + 1] += 1;
        }
    }

    println!("{}", a.iter().format(" "));
}
