use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }
    let mut ans = 0;
    for r in (1..=n).permutations(n) {
        if p < r && r < q {
            ans += 1;
        }
    }

    println!("{}", ans);
}
