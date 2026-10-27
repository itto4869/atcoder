use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s: Chars,
    }
    let ans = s.iter().position(|&c| c.is_uppercase()).unwrap() + 1;
    println!("{}", ans);
}
