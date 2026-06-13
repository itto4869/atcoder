use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let mut ans = Vec::new();
    for c in s {
        if c.is_digit(10) {
            ans.push(c);
        }
    }

    if ans.is_empty() {
    } else {
        println!("{}", ans.iter().format(""));
    }
}
