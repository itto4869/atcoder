use std::collections::HashSet;

use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut s: Chars,
        q: usize,
    }
    let mut shift = 0;
    let mut no_change = HashSet::new();
    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
            c: char,
        }
        if t == 1 {
            s[x - 1] = c;
            no_change.insert(x - 1);
        } else if t == 2 {
            shift = 1;
            no_change.clear();
        } else {
            shift = 2;
            no_change.clear();
        }
    }

    let mut ans = String::new();
    for i in 0..n {
        if no_change.contains(&i) || shift == 0 {
            ans.push(s[i]);
        } else if shift == 1 {
            ans.push(s[i].to_ascii_lowercase());
        } else {
            ans.push(s[i].to_ascii_uppercase());
        }
    }

    println!("{}", ans);
}
