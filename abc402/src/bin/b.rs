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
            c: u64,
        }

        if c == 1 {
            input! {
                x: u64,
            }
            queue.push_back(x);
        } else {
            println!("{}", queue.pop_front().unwrap());
        }
    }
}
