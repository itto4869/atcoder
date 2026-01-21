use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }
    let mut ans = 0;
    for comb in a.iter().combinations(2) {
        ans = ans.max(comb[0].abs_diff(*comb[1]));
    }

    println!("{}", ans);
}
