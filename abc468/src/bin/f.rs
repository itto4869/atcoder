use std::usize;

use cp_library::lis;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
    }
    let mut v2 = Vec::new();
    let mut max_v = 0;
    let mut ans = 0;
    for _ in 0..n {
        input! {
            p: usize,
        }
        if p > max_v {
            max_v = p;
            ans += 1;
        } else {
            v2.push(p);
        }
    }

    let lis = lis(&v2);
    ans += lis;
    println!("{}", ans);
}
