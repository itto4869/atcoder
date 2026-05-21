use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut cnt = vec![0; n];
    let mut v = Vec::new();
    for i in 1..=(3 * n) {
        input! {
            a: Usize1,
        }
        cnt[a] += 1;
        if cnt[a] == 2 {
            v.push((i, a + 1));
        }
    }

    v.sort_unstable();
    println!("{}", v.iter().map(|(_, a)| a).format(" "));
}
