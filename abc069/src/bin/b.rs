use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars
    }
    let n = s.len() - 2;
    println!("{}{}{}", s[0], n, s.last().unwrap());
}
