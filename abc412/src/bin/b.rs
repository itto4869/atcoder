use std::collections::HashSet;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
        t: Chars,
    }
    let mut set = HashSet::new();
    for c in t {
        set.insert(c);
    }

    for i in 1..s.len() {
        if s[i].is_uppercase() {
            if !set.contains(&s[i - 1]) {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
