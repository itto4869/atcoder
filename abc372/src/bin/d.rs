use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        h: [u64; n],
    }
    let mut stack = Vec::new();
    let mut ans = Vec::with_capacity(n);
    for &hi in h.iter().rev() {
        ans.push(stack.len());
        while let Some(top) = stack.pop() {
            if top > hi {
                stack.push(top);
                break;
            }
        }
        stack.push(hi);
    }

    println!("{}", ans.iter().rev().format(" "));
}
