use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut n: Chars,
    }
    if n[0] == n[2] {
        println!("Yes");
    } else {
        println!("No");
    }
}
