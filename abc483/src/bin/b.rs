use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut queue = VecDeque::new();
    for i in 1..=n {
        queue.push_back((i, 1i64));
    }

    for _ in 0..q {
        input! {
            k: usize,
        }
        let mut v = Vec::new();
        for _ in 0..k {
            let (idx, x) = queue.pop_front().unwrap();
            v.push((idx, x));
        }

        for (idx, x) in v {
            queue.push_front((idx, x * -1));
        }
    }

    let mut ans = Vec::new();
    for (idx, x) in queue {
        ans.push(idx as i64 * x);
    }

    println!("{}", ans.iter().format("\n"));
}
