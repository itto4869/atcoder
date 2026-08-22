use proconio::{fastout, input};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars
    }
    let mut ans = String::new();
    for c in s {
        if c == 'A' {
            ans.push(c);
        } else {
            ans.push('.');
        }
    }

    println!("{}", ans);
}
