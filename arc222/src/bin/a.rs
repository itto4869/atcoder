use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            mut lr: [(Usize1, Usize1); m],
        }
        lr.sort_unstable();
        let max_a = lr.iter().map(|(l, r)| r - l + 1).max().unwrap();
        let mut ans = Vec::new();
        let mut a = 1;
        for _ in 0..n {
            ans.push(a);
            if a == max_a {
                a = 1;
            } else {
                a += 1;
            }
        }

        println!("{}", ans.iter().format(" "));
    }
}
