use proconio::{fastout, input, marker::Bytes};

#[fastout]
fn main() {
    input! {
        s: Bytes,
    }
    if s[0] != s[1] && s[1] != s[2] && s[0] != s[2] {
        println!("Yes");
    } else {
        println!("No");
    }
}
