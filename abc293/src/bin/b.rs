use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut v = vec![false; n + 1];
    for i in 1..=n {
        input! {
            a: usize,
        }
        if !v[i] {
            v[a] = true;
        }
    }

    let mut ans = Vec::new();
    for i in 1..=n {
        if !v[i] {
            ans.push(i);
        }
    }

    println!("{}\n{}", ans.len(), ans.iter().format(" "));
}
