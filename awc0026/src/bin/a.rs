use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut ans = Vec::new();
    for i in 1..=n {
        input! {
            a: u64,
        }

        if i % k == 0 {
            ans.push(a);
        }
    }

    println!("{}", ans.iter().format(" "));
}
