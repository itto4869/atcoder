use proconio::{fastout, input};
use std::collections::HashMap;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut count = HashMap::new();
    for &ai in &a {
        *count.entry(ai).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (_, &num) in &count {
        if num % 2 == 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
