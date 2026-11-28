use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut ans = Vec::new();
    for c in s {
        if c == 'A' {
            ans.push("Alpha");
        } else if c == 'B' {
            ans.push("Bravo");
        } else {
            ans.push("Charlie");
        }
    }

    println!("{}", ans.iter().format(" "));
}
