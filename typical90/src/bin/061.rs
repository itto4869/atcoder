use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        }
        if t == 1 {
            queue.push_front(x);
        } else if t == 2 {
            queue.push_back(x);
        } else {
            println!("{}", queue.get(x - 1).unwrap());
        }
    }
}
