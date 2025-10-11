use proconio::{fastout, input};
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars
    }
    let middle = (s.len() + 1) / 2;
    for (i, c) in s.iter().enumerate() {
        if i == middle - 1 {
            continue;
        }
        print!("{}", c);
    }
}
