use itertools::{sorted};
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        x: [u64; n],
    }
    let sorted_x: Vec<u64> = sorted(x.to_vec()).collect();
    let median_l = sorted_x[n / 2 - 1];
    let median_r = sorted_x[n / 2];
    for i in 0..n {
        if x[i] <= median_l {
            println!("{}", median_r);
        } else {
            println!("{}", median_l);
        }
    }
}
