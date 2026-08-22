use std::collections::VecDeque;

use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        a: [usize; n],
    }
    let mut queue = VecDeque::new();
    let mut sum = 0;
    for i in 0..n {
        if queue.len() >= m {
            let d = queue.pop_front().unwrap();
            sum -= d;
        }

        let ai = a[i];

        if sum + ai <= k {
            sum += ai;
            queue.push_back(ai);
            println!("Yes");
        } else {
            queue.push_back(0);
            println!("No");
        }
    }
}
