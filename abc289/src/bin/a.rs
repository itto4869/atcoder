use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let s = s.into_iter().map(|c| {
        if c == '0' {
            '1'
        } else {
            '0'
        }
    }).collect::<String>();

    println!("{}", s);
}
