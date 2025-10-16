use std::collections::VecDeque;
use num::integer::lcm;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        t: [u128; n],
    }
    let mut t = VecDeque::from(t);
    while t.len() > 1 {
        let a = t.pop_front().unwrap();
        let b = t.pop_front().unwrap();
        let num = lcm(a, b);
        t.push_back(num);
    }
    println!("{}", t.pop_front().unwrap());
}
