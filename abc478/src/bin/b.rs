use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        v: usize,
        w: [usize; n],
    }
    let mut ans = 0;
    for comb in (0..n).combinations(3) {
        if comb.iter().sum::<usize>() <= (v - 3) {
            ans = ans.max(w[comb[0]] + w[comb[1]] + w[comb[2]]);
        }
    }

    println!("{}", ans);
}
