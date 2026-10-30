use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let mut a: Vec<usize> = a.into_iter().unique().collect();
    a.sort_unstable();
    let mut ans = 0;
    for x in 0..k {
        if a.len() == x {
            break;
        } else if a[x] != x {
            break;
        } else {
            ans = x + 1;
        }
    }

    println!("{}", ans);
}
