use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let ans = s.into_iter().filter(|x| x.is_ascii_uppercase()).count();
    println!("{}", ans);
}
