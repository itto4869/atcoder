use std::collections::VecDeque;

use ac_library::{Mod998244353, StaticModInt};
use proconio::{fastout, input};

type Mod998 = StaticModInt<Mod998244353>;
#[fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut ans = Mod998::new(1);
    let mut queue = VecDeque::new();
    queue.push_back(1);
    for _ in 0..q {
        input! {
            t: usize,
        }
        if t == 1 {
            input! {
                x: usize,
            }
            ans = ans * 10 + Mod998::new(x);
            queue.push_back(x);
        } else if t == 2 {
            let top = queue.pop_front().unwrap();
            ans = ans - Mod998::new(top) * Mod998::new(10).pow(queue.len() as u64);
        } else {
            println!("{}", ans);
        }
    }
}
