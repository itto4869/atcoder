use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        d: [u64; n - 1],
    }
    let mut ans = Vec::with_capacity(n);
    for i in 0..(n - 1) {
        let mut dist = 0;
        for j in i..(n - 1) {
            dist += d[j];
            ans.push(dist);
        }
        println!("{}", ans.iter().format(" "));
        ans.clear();
    }
}
