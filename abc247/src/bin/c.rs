use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let ans = s(n);
    println!("{}", ans.iter().format(" "));
}

fn s(n: usize) -> Vec<usize> {
    if n == 1 {
        return [1].to_vec();
    } else {
        let mut v = s(n - 1);
        v.push(n);
        v.append(&mut s(n - 1));
        return v;
    }
}